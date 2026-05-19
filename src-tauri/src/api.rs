use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const USER_AGENT: &str = "GhostWriter/0.1.0";

#[derive(Debug, Serialize, PartialEq)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, PartialEq)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ChatResponse {
    choices: Vec<Choice>,
}

pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

fn build_chat_request(model: &str, instruction: &str, text: &str) -> ChatRequest {
    ChatRequest {
        model: model.to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: instruction.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ],
        temperature: 0.7,
    }
}

fn extract_response(text: &str) -> Result<String, String> {
    let chat_response: ChatResponse =
        serde_json::from_str(text).map_err(|e| format!("Parse error: {}", e))?;
    match chat_response.choices.into_iter().next() {
        Some(choice) => Ok(choice.message.content),
        None => Err("No response from API".to_string()),
    }
}

impl OpenRouterClient {
    pub fn new(api_key: String, model: String, proxy_url: Option<String>) -> Result<Self, String> {
        let mut builder = Client::builder()
            .timeout(Duration::from_secs(120));

        if let Some(ref proxy_str) = proxy_url {
            match reqwest::Proxy::all(proxy_str) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                    log::info!("Using proxy: {}", proxy_str);
                }
                Err(e) => {
                    log::warn!("Invalid proxy URL '{}': {}, ignoring", proxy_str, e);
                }
            }
        }

        let client = builder
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            api_key,
            model,
            base_url: OPENROUTER_URL.to_string(),
        })
    }

    pub fn new_with_url(
        api_key: String,
        model: String,
        proxy_url: Option<String>,
        base_url: String,
    ) -> Result<Self, String> {
        let mut client = Self::new(api_key, model, proxy_url)?;
        client.base_url = base_url;
        Ok(client)
    }

    pub async fn refine_text(
        &self,
        instruction: &str,
        text: &str,
    ) -> Result<String, String> {
        let request = build_chat_request(&self.model, instruction, text);

        let response = match self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/arvinmoj/GhostWriter")
            .header("X-Title", "GhostWriter")
            .header("User-Agent", USER_AGENT)
            .json(&request)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("API error: {} - {}", status, body);
            return Err(format!("API error: {} - {}", status, body));
        }

        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        extract_response(&body)
    }
}
