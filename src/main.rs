use clap::Parser;
use reqwest;
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use std::{
    fs,
    path::{self},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    output: path::PathBuf,

    #[clap(short, long)]
    api_key: String,

    #[clap(short, long)]
    prompt: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    id: String,
    created: i32,
    model: String,
    choices: Vec<Choice>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {:?}",
            self.id,
            self.created,
            self.model,
            self.choices.iter().map(|c| c.text.to_string())
        )
    }
}

#[tokio::main]
async fn main() {
    // format cli args
    let args = Args::parse();

    let client = reqwest::Client::new();

    let endpoint = String::from("https://api.openai.com/v1/completions");
    let token = format!("Bearer {}", args.api_key);
    let prompt = args.prompt.unwrap_or(String::from(
        "Generate three jeopardy questions about Africa",
    ));

    let body = json!({
        "model": "text-davinci-002",
        "prompt": prompt,
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

    // write to output file
    match fs::write(&args.output, response.to_string()) {
        Ok(_) => println!("Success."),
        Err(e) => println!(
            "Encountered the following error when writing to output {}",
            e
        ),
    };
}
