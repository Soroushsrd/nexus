pub mod chains;
pub mod embeddings;

use reqwest::Client;
use serde::{Deserialize,Serialize};
use std::env;
use tokio;
use dotenv::dotenv;


#[derive(Serialize, Deserialize,PartialEq,Debug)]
pub struct Message {
    role : String,
    content: String,
}

#[derive(Serialize)]
pub struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64
}

#[derive(Deserialize)]
pub struct MessageResponse {
    content: String
}

#[derive(Deserialize)]
pub struct Choice {
    index: u32,
    message: MessageResponse,
    finish_reason: String,
}

#[derive(Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
#[derive(Deserialize)]
pub struct  ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model:String,
    choices: Vec<Choice>,
    usage: Usage,
}

impl ChatCompletionResponse {
    pub fn usage(&self) -> &Usage{
        &self.usage
    }
    pub fn choices(&self) -> &Vec<Choice> {
        &self.choices
    }

}

impl Usage {
    pub fn prompt_tokens(&self) ->u32{
        self.prompt_tokens
    }
    pub fn completion_tokens(&self) ->u32{
        self.completion_tokens
    }
    pub fn total_tokens(&self) ->u32{
        self.total_tokens
    }
}
pub async fn completion(api_key: &str, messages: Vec<Message>, temperature: f64) -> Result<String, Box<dyn std::error::Error>> {
    if temperature < 0.0 || temperature > 1.0 {
        panic!("Temperature must be between 0 and 1 and written with decimals. \n for example: 0.5");
    }

    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: "gpt-4o-mini".to_string(),
        messages,
        temperature
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;

    let response_body: ChatCompletionResponse = serde_json::from_str(&response_text)?;

    if let Some(choice) = response_body.choices.first() {
        Ok(choice.message.content.clone())
    } else {
        Err("No response choices received!".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_send_request() {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "you are a helpful AI assistant".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "what are LLMs?".to_string(),
            },
        ];

        let result = completion(&api_key, messages,0.5).await;
        assert!(result.is_ok());
        println!("Response: {}", result.unwrap());
    }
}
