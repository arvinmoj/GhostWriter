use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const USER_AGENT: &str = "GhostWriter/0.1.0";

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String, model: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, api_key, model }
    }

    pub async fn refine_text(
        &self,
        instruction: &str,
        text: &str,
    ) -> Result<String, String> {
        let request = ChatRequest {
            model: self.model.clone(),
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
        };

        let response = match self.client
            .post(OPENROUTER_URL)
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

        let chat_response: ChatResponse = match response.json().await {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Parse error: {}", e)),
        };

        match chat_response.choices.first() {
            Some(choice) => Ok(choice.message.content.clone()),
            None => Err("No response from API".to_string()),
        }
    }
}
