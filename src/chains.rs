use crate::{completion, Message};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

pub struct Prompt {
    template: String,
    variables: HashMap<String, String>,
    role: String,
}

impl Prompt {
    pub fn new(template: String, role: String) -> Self {
        let variables = HashMap::new();
        Prompt {
            template,
            variables,
            role,
        }
    }

    pub fn add_variable(mut self, key: String, value: String) -> Self {
        self.variables.insert(key, value);
        self
    }

    pub fn render(&self) -> String {
        let mut rendered = self.template.clone();
        for (key, value) in &self.variables {
            let mut placeholder = String::new();
            placeholder.push_str("{");
            placeholder.push_str(key);
            placeholder.push_str("}");
            rendered = rendered.replace(&placeholder, value);
        }
        rendered
    }
    pub fn to_message(&self) -> Message {
        Message {
            role: self.role.clone(),
            content: self.render(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt() {
        let mut temp = Prompt::new(
            "hello {world} {information}".to_string(),
            "user".to_string(),
        )
        .add_variable("world".to_string(), ",World".to_string())
        .add_variable(
            "information".to_string(),
            "armenian rugs are different than persian rugs".to_string(),
        );

        let expected_output =
            "hello ,World armenian rugs are different than persian rugs".to_string();

        assert_eq!(temp.render(), expected_output);
    }

    #[test]
    fn test_prompt_to_message() {
        let template =
            "With the information provided, answer the question: \n {question} \n {information}";
        let expected_message = Message {
            role: "user".to_string(),
            content: "With the information provided, answer the question: \n tell me about armenian rugs \n armenian rugs are different than persian rugs".to_string(),
        };
        let prompt = Prompt::new(template.to_string(), "user".to_string())
            .add_variable(
                "question".to_string(),
                "tell me about armenian rugs".to_string(),
            )
            .add_variable(
                "information".to_string(),
                "armenian rugs are different than persian rugs".to_string(),
            );
        assert_eq!(prompt.to_message(), expected_message);
    }

    #[tokio::test]
    async fn test_chain() {
        dotenv().ok();

        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");

        let system_prompt = Prompt::new(
            "you are a helpful AI assistant".to_string(),
            "system".to_string(),
        );

        let template =
            "With the information provided, answer the question: \n {question} \n {information}";
        let user_prompt = Prompt::new(template.to_string(), "user".to_string())
            .add_variable(
                "question".to_string(),
                "tell me about armenian rugs".to_string(),
            )
            .add_variable(
                "information".to_string(),
                "armenian rugs are different than persian rugs".to_string(),
            );

        let messages = vec![system_prompt.to_message(), user_prompt.to_message()];

        let result = completion(&api_key, messages, 0.5).await;

        println!("Response: {}", result.unwrap());
    }
}
