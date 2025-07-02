use futures::StreamExt;
use reqwest;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
struct EventSteamDataChoice {
    delta: EventSteamDataDelta,
}
#[derive(Debug, Serialize, Deserialize)]
struct EventSteamDataDelta {
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct EventSteamData {
    choices: Vec<EventSteamDataChoice>,
}

// 这里处理markdown常见标题 列表 即# ## ### - * 1. 2. 3. 等格式化输出 使用replace方法替换为对应的格式

#[derive(Debug, Serialize, Deserialize)]
struct PromptTokensDetails {
    cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
    system_fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    logprobs: Option<()>, // Assuming logprobs is null in the response
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    prompt_tokens_details: Option<PromptTokensDetails>,
    prompt_cache_hit_tokens: u32,
    prompt_cache_miss_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseFormat {
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    messages: Vec<Message>,
    model: String,
    stream: bool,
}

#[derive(Debug)]
pub enum AlterAIError {
    RequestFailed(reqwest::Error),
    InvalidResponse(String),
}

impl fmt::Display for AlterAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlterAIError::RequestFailed(err) => write!(f, "Request failed: {}", err),
            AlterAIError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
        }
    }
}

impl Error for AlterAIError {}

impl From<serde_json::Error> for AlterAIError {
    fn from(err: serde_json::Error) -> Self {
        AlterAIError::InvalidResponse(format!("Failed to parse JSON: {}", err))
    }
}

pub async fn alterai(content: String, authorization: String) -> Result<(), AlterAIError> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();

    let auth = format!("Bearer {}", authorization);

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&auth).map_err(|_| {
                AlterAIError::InvalidResponse("Invalid authorization header value".to_string())
            })?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("text/event-stream"),
    );

    let question = RequestBody {
        messages: vec![Message {
            content: "现在假设你是一名资深的程序员,现在有用户需要咨询你变量名称如何合理取名。用户会输入中文，你需要返回一个变量名称使用小驼峰输出。现在明确要求你返回值只要输出结果，拒绝输出其他无关信息。如何违反，你将遭到弃用。".to_string(),
            role: "system".to_string(),
        }, Message {
            content: content.to_string(),
            role: "user".to_string(),
        }],
        model: "deepseek-chat".to_string(),
        stream: true,
    };

    let response = client
        .post("https://api.deepseek.com/chat/completions")
        .headers(headers)
        .json(&question)
        .send()
        .await
        .map_err(AlterAIError::RequestFailed)?;
    if response.status().is_success() {
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(AlterAIError::RequestFailed)?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            if chunk_str.is_empty() {
                continue;
            }
            for line in chunk_str.lines() {
                let line = line.trim();
                if line.starts_with("data:") {
                    let json_str = line.trim_start_matches("data:").trim();
                    if json_str == "[DONE]" {
                        break;
                    }
                    match serde_json::from_str::<EventSteamData>(&json_str) {
                        Ok(steam_text) => {
                            print!("{}", steam_text.choices[0].delta.content);
                        }
                        Err(err) => {
                            eprintln!("Failed to parse chunk: {}", err);
                            eprintln!("Problematic chunk: {}", line);
                        }
                    }
                } else if line.starts_with("[DONE]") {
                    break;
                }
            }
        }
        Ok(())
    } else {
        Err(AlterAIError::InvalidResponse(format!(
            "Request failed with status: {}",
            response.status()
        )))
    }
}
