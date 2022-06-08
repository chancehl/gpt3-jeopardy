use reqwest;
use serde_json::json;
use serde::{Deserialize};
use std::env;

#[derive(Deserialize, Debug)]
struct Choice {
    text: String
}

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    created: i32,
    model: String,
    choices: Vec<Choice>
}

#[tokio::main]
async fn main() {
    let api_key = env::var("OPEN_AI_API_KEY").expect("$OPEN_AI_API_KEY is not set");

    let client = reqwest::Client::new();

    let endpoint = String::from("https://api.openai.com/v1/completions");
    let token = format!("Bearer {}", api_key);

    let body = json!({
        "model": "text-davinci-002",
        "prompt": "Generate three jeopardy questions about Ronald Regan",
        "temperature": f32::from(0.7),
        "max_tokens": i32::from(256)
    });

    let request = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("authorization", token)
        .body(body.to_string());

    let response = match request.send().await {
        Ok(resp) => resp.json::<Response>().await.unwrap(),
        Err(err) => {
            println!("Request failed: {}", err.to_string());
            return;
        }
    };

    println!("{:?}", response);
}
