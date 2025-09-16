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

    let body = serde_json::json!({
        "model": "gpt-5.0-chat", 
        "instructions": "you are a terse assistant.",
        "input": [
            {
                "type":"message",
                "role":"user",
                "content":[
                    { "type": "output_text", "text": "hello world" }
                ]
            }
        ],
        "tools": [],
        "tool_choice": "auto",
        "parallel_tool_calls": false,
        "reasoning": serde_json::Value::Null,
        "store": true,
        "stream": true,
        "include": [],
        "prompt_cache_key": conv_id,
        "text": serde_json::Value::Null
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(access_token)
        .header("chatgpt-account-id", account_id)
        .header("accept", "text/event-stream")
        .header("content-type", "application/json")
        .header("openai-beta", "responses=experimental")
        .header("conversation_id", &conv_id)
        .header("session_id", &conv_id)
        .json(&body)
        .send()
        .await?;

    println!("status: {}", res.status());
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
