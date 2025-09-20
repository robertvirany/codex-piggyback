use serde::Deserialize;
use std::{fs, path::PathBuf};
use uuid::Uuid;
use futures_util::StreamExt;

#[derive(Deserialize)]
struct AuthJson {
    tokens: Tokens,
}

#[derive(Deserialize)]
struct Tokens {
    access_token: String,
    account_id: String,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read ~/.codex/auth.json
    let mut path = PathBuf::from(std::env::var("HOME")?);
    path.push(".codex/auth.json");
    let data = fs::read_to_string(path)?;
    let auth: AuthJson = serde_json::from_str(&data)?;

    let access_token = &auth.tokens.access_token;
    let account_id = &auth.tokens.account_id;

    let conv_id = Uuid::new_v4().to_string();
    
    const BASE_PROMPT: &str = include_str!("prompt.md");
    let prompt_cache_key = conv_id.clone();
    //let instructions = format!("{BASE_PROMPT}\n\nYou are a terse assistant. Who was the first president of the USA.");

    let body = serde_json::json!({
        "model": "gpt-5", 
        "instructions": BASE_PROMPT,
        "input": [
            {
                "type":"message",
                "role":"user",
                "content":[
                    { "type": "input_text", "text": "hello world" }
                ]
            }
        ],
        "tools": [],
        "tool_choice": "auto",
        "parallel_tool_calls": false,
        "reasoning": {"summary":"auto"},
        "store": false,
        "stream": true,
        "include": ["reasoning.encrypted_content"],
        "prompt_cache_key": prompt_cache_key,
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://chatgpt.com/backend-api/codex/responses")
        .bearer_auth(access_token)
        //.header("version", "0.34.0")
        .header("openai-beta", "responses=experimental")
        .header("conversation_id", &conv_id)
        .header("session_id", &conv_id)
        .header("accept", "text/event-stream")
        .header("content-type", "application/json")
        .header("chatgpt-account-id", account_id)
        //.header("user-agent", "codex_cli_rs/0.34.0 (Ubuntu 24.4.0; x86_64) WindowsTerminal")
        //.header("originator", "codex_cli_rs")
        //.header("host", "chatgpt.com")
        //.header("content-length", //CONTENT-LENGTH//)
        .json(&body)
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("{}: {}", res.status(), res.text().await?);
        return Ok(());
    }
    let mut stream = res.bytes_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    print!("{text}");
                }
            }
            Err(e) => {
                eprintln!("stream error: {e}");
                break;
            }
        }
    }

    Ok(())
}
