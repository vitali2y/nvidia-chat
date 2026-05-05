// Simple Rust CLI AI chat via Nvidia AI API

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{self, Read},
    process,
};

const NVIDIA_API_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

// default fallback model
// under NVIDIA_MODEL env var you can use e. g. "z-ai/glm-4.7", "qwen/qwen3-next-80b-a3b-instruct", "stepfun-ai/step-3.5-flash" models
const MODEL_NAME: &str = "z-ai/glm-5.1";

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct ApiRequest {
    model: String,
    messages: Vec<ChatMessage>,
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
    let prompt = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|e| format!("failed to read stdin: {}", e))?;
            input
        }
    };

    if prompt.trim().is_empty() {
        eprintln!("usage: `nvidia-chat \"your question\"` or `cat file | nvidia-chat`");
        process::exit(1);
    }

    let api_key = match env::var("NVIDIA_API_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("error: NVIDIA_API_KEY env var not set");
            process::exit(1);
        }
    };

    let model = env::var("NVIDIA_MODEL").unwrap_or_else(|_| MODEL_NAME.to_string());

    let request_payload = ApiRequest {
        model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = Client::new();

    let response = match client
        .post(NVIDIA_API_URL)
        .bearer_auth(api_key)
        .json(&request_payload)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("network error: {}", e);
            process::exit(2);
        }
    };

    if response.status().is_success() {
        let api_response: ApiResponse = match response.json().await {
            Ok(json) => json,
            Err(e) => {
                eprintln!("failed to parse API response: {}", e);
                process::exit(2);
            }
        };

        if let Some(choice) = api_response.choices.first() {
            println!("{}", choice.message.content.trim());
        } else {
            eprintln!("error: empty response choices from API");
            process::exit(3);
        }
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "<no body>".to_string());
        eprintln!("API error ({}): {}", status, error_text);
        process::exit(4);
    }

    Ok(())
}
