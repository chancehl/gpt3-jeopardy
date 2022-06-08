use reqwest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPEN_AI_API_KEY").expect("$OPEN_AI_API_KEY is not set");

    let client = reqwest::Client::new();

    let endpoint = String::from("https://api.openai.com/v1/completions");
    let token = format!("Bearer {}", api_key);

    let body = json!({
        "model": "text-davinci-002",
        "prompt": "Generate three jeopardy questions about Ronald Regan",
        "temperature": 0.7,
        "max_tokens": 256
    });

    let request = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("authorization", token)
        .body(body.to_string());

    let response = match request.send().await {
        Ok(resp) => resp.text().await,
        Err(err) => {
            println!("Request failed: {}", err.to_string());
            return;
        }
    };

    println!("{:?}", response);
}
