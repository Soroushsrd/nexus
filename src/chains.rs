use std::collections::HashMap;
use crate::Message;

pub trait Step {
    fn execute(&self, input: &str) -> String;
}

pub struct Chain {
    steps: Vec<Box<dyn Step>>
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            steps: Vec::new(),
        }
    }
    pub fn add_steps(mut self, step: Box<dyn Step>) -> Self{
        self.steps.push(step);
        self
    }
    pub fn execute(&self, input: &str)-> String{
        let mut output = input.to_string();
        for step in &self.steps {
            output = step.execute(&output);
        }
        output
    }
    
}

pub struct Prompt {
    template: String,
    variables: HashMap<String,String>
}

impl Prompt {
    pub fn new(template: String) -> Self{
        let  variables  = HashMap::new();
        Prompt{
            template,
            variables,
        }
    }

    pub fn add_variable(mut self, key:String,value:String)-> Self{
        self.variables.insert(key, value);
        self
    }
    // pub fn render(&self,) -> String{
    //     let mut rendered = self.template.clone();
    //     for (key, value) in &self.variables {
    //         let placeholder = format!(rendered, key);
    //         rendered = rendered.replace(&placeholder, value);
    //     }
    //     rendered
    // }
    pub fn render(&self) -> String {
        let mut rendered = self.template.clone();
        for (key, value) in &self.variables {
            let mut placeholder = String::new();
            placeholder.push_str("{");
            placeholder.push_str(key);
            placeholder.push_str("}");
            println!("Placeholder: {}", placeholder);
            println!("Before replace: {}", rendered);
            rendered = rendered.replace(&placeholder, value);
            println!("After replace: {}", rendered);
        }
        rendered
    }
    pub fn to_message(&self) -> Message{
        Message{
            role: "user".to_string(),
            content: self.render()
        }
    }
}

impl Step for Prompt {
     fn execute(&self, _input: &str) -> String {
        self.render()
    }
}

// llm struct should be implemented here.
// should probably have a url, temp, api key and model
pub struct LLM; // just a dummy!

impl Step for LLM {
     fn execute(&self, input: &str) -> String {
        format!("OutputL {}", input)
    }
    
}


//might need a parser for the output here. not sure yet
// serde crate does take care of a huge part out outputs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt(){
        let mut temp = Prompt::new("hello {world} {information}".to_string())
            .add_variable("world".to_string(), ",World".to_string())
            .add_variable("information".to_string(),"armenian rugs are different than persian rugs".to_string());

        let expected_output= "hello ,World armenian rugs are different than persian rugs".to_string();

        assert_eq!(temp.render(),expected_output);

    }

    #[test]
    fn test_prompt_to_message() {
        let template = "With the information provided, answer the question: \n {question} \n {information}";
        let expected_message = Message {
            role: "user".to_string(),
            content: "With the information provided, answer the question: \n tell me about armenian rugs \n armenian rugs are different than persian rugs".to_string(),
        };
        let prompt = Prompt::new(template.to_string())
            .add_variable("question".to_string(), "tell me about armenian rugs".to_string())
            .add_variable("information".to_string(), "armenian rugs are different than persian rugs".to_string());
        assert_eq!(prompt.to_message(), expected_message);
    }
}
