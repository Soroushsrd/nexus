use qdrant_client::Qdrant;
use tokio;
use qdrant_client::qdrant::SearchPointsBuilder;
use serde::{Serialize,Deserialize};
use serde_json::json;
use dotenv::dotenv;
use reqwest::Client;
use std::env;


#[derive(Serialize,Deserialize,Debug)]
pub struct EmbeddingResponse {
    object: String,
    model: String,
    usage: Usage,
    data: EmbeddingObject,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Usage {
    prompt_tokens : u32,
    total_tokens : u32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct EmbeddingObject {
    object : String,
    index : u32,
    embedding : Vec<f32>
}


#[tokio::main]
pub async fn query_db(query: Vec<f32>, collection: &str) -> Result<(), Box< dyn std::error::Error>> {
    let client = Qdrant::from_url("https://localhost:6333").build()?;

    let search_request = SearchPointsBuilder::new(
        collection,
        query,
        4
    ).build();

    match client.search_points(search_request).await {
        Ok(_response) => Ok(()),
        Err(e) => panic!("couldn't get a response: {}", e)
    }
}


// a function to create embeddings out of queries shoudl be here
pub async fn generate_embedding_vector(question: &str) -> Vec<f32> {
    dotenv().ok();
    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("Wasnt able to read the key");
    let url = "https://api.openai.com/v1/embeddings";

    let request_body = json!({
        "input": question,
        "model": "text-embedding-3-small"
    });

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .unwrap()
        .json::<EmbeddingResponse>()
        .await
        .unwrap();

    let embedding = response.data.embedding;
    embedding
}

// write the retriever
//write the prompts
//write chat history pass er
//write chat completion
