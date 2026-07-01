use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
const USER_AGENT: &str = "GhostWriter/0.1.0";

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Gemini response types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

// ---------------------------------------------------------------------------
// API client enum — routes to the right implementation
// ---------------------------------------------------------------------------

pub enum ApiClient {
    OpenRouter(OpenRouterClient),
    Google(GeminiClient),
}

impl ApiClient {
    pub async fn refine_text(&self, instruction: &str, text: &str) -> Result<String, String> {
        match self {
            ApiClient::OpenRouter(c) => c.refine_text(instruction, text).await,
            ApiClient::Google(c) => c.refine_text(instruction, text).await,
        }
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

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

fn strip_thinking_tags(mut content: String) -> String {
    while let Some(start) = content.find("<think>") {
        if let Some(end) = content[start..].find("</think>") {
            content.replace_range(start..start + end + 8, "");
        } else {
            content.replace_range(start..start + 7, "");
        }
    }
    content.trim().to_string()
}

fn extract_response(text: &str) -> Result<String, String> {
    let chat_response: ChatResponse =
        serde_json::from_str(text).map_err(|e| format!("Parse error: {}", e))?;
    match chat_response.choices.into_iter().next() {
        Some(choice) => Ok(strip_thinking_tags(choice.message.content)),
        None => Err("No response from API".to_string()),
    }
}

fn extract_gemini_response(text: &str) -> Result<String, String> {
    let response: GeminiResponse =
        serde_json::from_str(text).map_err(|e| format!("Parse error: {}", e))?;
    match response.candidates.into_iter().next() {
        Some(candidate) => {
            let combined: String = candidate
                .content
                .parts
                .into_iter()
                .filter_map(|p| p.text)
                .collect();
            if combined.is_empty() {
                Err("No response from API".to_string())
            } else {
                Ok(strip_thinking_tags(combined))
            }
        }
        None => Err("No response from API".to_string()),
    }
}

// ---------------------------------------------------------------------------
// OpenRouter / OpenAI-compatible client
// ---------------------------------------------------------------------------

pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String, model: String, proxy_url: Option<String>) -> Result<Self, String> {
        let mut builder = Client::builder().timeout(Duration::from_secs(120));

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

    pub async fn refine_text(&self, instruction: &str, text: &str) -> Result<String, String> {
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

// ---------------------------------------------------------------------------
// Google Gemini client
// ---------------------------------------------------------------------------

pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiClient {
    pub fn new(api_key: String, model: String, proxy_url: Option<String>) -> Result<Self, String> {
        let mut builder = Client::builder().timeout(Duration::from_secs(120));

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
        })
    }

    pub async fn refine_text(&self, instruction: &str, text: &str) -> Result<String, String> {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            GEMINI_BASE_URL, self.model, self.api_key
        );

        let body = serde_json::json!({
            "contents": [
                {
                    "role": "user",
                    "parts": [{ "text": text }]
                }
            ],
            "systemInstruction": {
                "parts": [{ "text": instruction }]
            },
            "generationConfig": {
                "temperature": 0.7
            }
        });

        let response = match self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("User-Agent", USER_AGENT)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!("Gemini API error: {} - {}", status, body);
            return Err(format!("Gemini API error: {} - {}", status, body));
        }

        let response_body = match response.text().await {
            Ok(b) => b,
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        extract_gemini_response(&response_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Chat request helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_build_chat_request() {
        let req = build_chat_request("gpt-4", "Be concise.", "Hello world");
        assert_eq!(req.model, "gpt-4");
        assert_eq!(req.messages.len(), 2);
        assert_eq!(req.messages[0].role, "system");
        assert_eq!(req.messages[0].content, "Be concise.");
        assert_eq!(req.messages[1].role, "user");
        assert_eq!(req.messages[1].content, "Hello world");
        assert_eq!(req.temperature, 0.7);
    }

    // -----------------------------------------------------------------------
    // OpenRouter response extraction
    // -----------------------------------------------------------------------

    #[test]
    fn test_extract_response_success() {
        let json = r#"{"choices":[{"message":{"content":"Refined text"}}]}"#;
        let result = extract_response(json).unwrap();
        assert_eq!(result, "Refined text");
    }

    #[test]
    fn test_extract_response_empty_choices() {
        let json = r#"{"choices":[]}"#;
        let result = extract_response(json);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No response from API");
    }

    #[test]
    fn test_extract_response_malformed_json() {
        let result = extract_response("not json");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Parse error"));
    }

    #[test]
    fn test_extract_response_missing_message() {
        let json = r#"{"choices":[{"foo":"bar"}]}"#;
        let result = extract_response(json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Parse error"));
    }

    // -----------------------------------------------------------------------
    // Gemini response extraction
    // -----------------------------------------------------------------------

    #[test]
    fn test_extract_gemini_response_success() {
        let json = r#"{"candidates":[{"content":{"parts":[{"text":"Refined text"}]}}]}"#;
        let result = extract_gemini_response(json).unwrap();
        assert_eq!(result, "Refined text");
    }

    #[test]
    fn test_extract_gemini_response_multiple_parts() {
        let json = r#"{"candidates":[{"content":{"parts":[{"text":"Hello "},{"text":"world"}]}}]}"#;
        let result = extract_gemini_response(json).unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_extract_gemini_response_empty_candidates() {
        let json = r#"{"candidates":[]}"#;
        let result = extract_gemini_response(json);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No response from API");
    }

    #[test]
    fn test_extract_gemini_response_empty_text() {
        let json = r#"{"candidates":[{"content":{"parts":[{"text":""}]}}]}"#;
        let result = extract_gemini_response(json);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No response from API");
    }

    #[test]
    fn test_extract_gemini_response_with_thinking_tags() {
        let json =
            r#"{"candidates":[{"content":{"parts":[{"text":"<think>reasoning</think>Answer"}]}}]}"#;
        let result = extract_gemini_response(json).unwrap();
        assert_eq!(result, "Answer");
    }

    #[test]
    fn test_extract_gemini_response_malformed_json() {
        let result = extract_gemini_response("not json");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Parse error"));
    }

    // -----------------------------------------------------------------------
    // OpenRouter client
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_refine_text_network_error() {
        let client = OpenRouterClient {
            client: Client::builder().build().unwrap(),
            api_key: "test".to_string(),
            model: "test".to_string(),
            base_url: "http://localhost:1/api/v1/chat/completions".to_string(),
        };
        let err = client
            .refine_text("Be concise.", "Hello world")
            .await
            .unwrap_err();
        assert!(err.contains("Network error") || err.contains("error"));
    }

    #[tokio::test]
    async fn test_new_with_url_sets_base_url() {
        let client = OpenRouterClient::new_with_url(
            "key".to_string(),
            "model".to_string(),
            None,
            "http://custom.local/api".to_string(),
        )
        .unwrap();
        assert_eq!(client.base_url, "http://custom.local/api");
    }

    #[test]
    fn test_chat_request_partial_eq() {
        let req1 = build_chat_request("gpt-4", "Fix grammar.", "hello");
        let req2 = build_chat_request("gpt-4", "Fix grammar.", "hello");
        assert_eq!(req1, req2);
    }

    // -----------------------------------------------------------------------
    // Gemini client
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_gemini_refine_text_network_error() {
        let client = GeminiClient {
            client: Client::builder().build().unwrap(),
            api_key: "test".to_string(),
            model: "gemini-2.0-flash".to_string(),
        };
        let err = client
            .refine_text("Be concise.", "Hello world")
            .await
            .unwrap_err();
        assert!(err.contains("Network error") || err.contains("error"));
    }

    #[test]
    fn test_gemini_client_creates_without_proxy() {
        let result = GeminiClient::new("key".to_string(), "gemini-2.0-flash".to_string(), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gemini_client_creates_with_valid_proxy() {
        let result = GeminiClient::new(
            "key".to_string(),
            "gemini-2.0-flash".to_string(),
            Some("http://127.0.0.1:8080".to_string()),
        );
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // ApiClient enum routing
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_api_client_openrouter_route() {
        let client = ApiClient::OpenRouter(OpenRouterClient {
            client: Client::builder().build().unwrap(),
            api_key: "test".to_string(),
            model: "test".to_string(),
            base_url: "http://localhost:1/api/v1/chat/completions".to_string(),
        });
        let err = client.refine_text("Be concise.", "Hello world").await;
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(msg.contains("Network error") || msg.contains("error"));
    }

    #[tokio::test]
    async fn test_api_client_gemini_route() {
        let client = ApiClient::Google(GeminiClient {
            client: Client::builder().build().unwrap(),
            api_key: "test".to_string(),
            model: "gemini-2.0-flash".to_string(),
        });
        let err = client.refine_text("Be concise.", "Hello world").await;
        assert!(err.is_err());
        let msg = err.unwrap_err();
        assert!(msg.contains("Network error") || msg.contains("error"));
    }

    // -----------------------------------------------------------------------
    // Thinking tag stripping
    // -----------------------------------------------------------------------

    #[test]
    fn test_strip_thinking_tags_removes_full_tag() {
        let result = strip_thinking_tags("<think>some reasoning</think>Hello".to_string());
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_strip_thinking_tags_no_tags() {
        let result = strip_thinking_tags("Just a normal response".to_string());
        assert_eq!(result, "Just a normal response");
    }

    #[test]
    fn test_strip_thinking_tags_multiple() {
        let result = strip_thinking_tags(
            "<think>step 1</think>Answer<think>step 2</think> here".to_string(),
        );
        assert_eq!(result, "Answer here");
    }

    #[test]
    fn test_strip_thinking_tags_unclosed() {
        let result = strip_thinking_tags("<think>no close tag".to_string());
        assert_eq!(result, "no close tag");
    }

    #[test]
    fn test_strip_thinking_tags_with_whitespace() {
        let result = strip_thinking_tags("  <think>reasoning</think>  Final output  ".to_string());
        assert_eq!(result, "Final output");
    }
}
