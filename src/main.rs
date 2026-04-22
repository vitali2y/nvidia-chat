// Simple Rust CLI AI chat via Nvidia AI API

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{self, Read},
    process,
};

const NVIDIA_API_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

// use any of models below
// TODO: to have possibility to select any (NVIDIA_MODEL var?)
const MODEL_NAME: &str = "z-ai/glm-5.1";
// const MODEL_NAME: &str = "z-ai/glm-4.7";
// const MODEL_NAME: &str = "qwen/qwen3-next-80b-a3b-instruct";
// const MODEL_NAME: &str = "stepfun-ai/step-3.5-flash";

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ApiRequest {
    model: String,
    messages: Vec<ChatMessage>,
    // optional specific tokens limit might be specified below
    // max_tokens: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ChatMessage,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut prompt = String::new();
    if args.len() == 2
    // prompt as an arg
    {
        prompt = args[1].clone()
    } else {
        // prompt as a stdin pipe
        io::stdin()
            .read_to_string(&mut prompt)
            .expect("failed to read stdin");
    }

    if prompt.trim().is_empty() {
        eprintln!("usage: `nvidia-chat \"your question\"` or `cat file | nvidia-chat`");
        process::exit(1);
    }

    let api_key = env::var("NVIDIA_API_KEY").expect("NVIDIA_API_KEY env var not set!");

    // predefined system message might be specified for coding context
    // messages: vec![
    //     ChatMessage { role: "system".to_string(), content: "You are a coding assistant.".to_string() },
    //     ChatMessage { role: "user".to_string(), content: prompt }
    // ]
    let request_payload = ApiRequest {
        model: MODEL_NAME.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    // sending request
    let client = Client::new();
    let response = client
        .post(NVIDIA_API_URL)
        .bearer_auth(api_key)
        .json(&request_payload)
        .send()
        .await
        .expect("failed to send request to NVIDIA API");

    // handling the response
    if response.status().is_success() {
        let api_response = response
            .json::<ApiResponse>()
            .await
            .expect("failed to parse API response");

        if let Some(first_choice) = api_response.choices.get(0) {
            println!("{}", first_choice.message.content.trim());
        } else {
            eprintln!("error: empty response choices from API");
            process::exit(2);
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        eprintln!("API error ({}): {}", status, error_text);
        process::exit(3);
    }

    Ok(())
}
