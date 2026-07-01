use ghostwriter_lib::api::{GeminiClient, OpenRouterClient};

// ---------------------------------------------------------------------------
// OpenRouter client creation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_openrouter_client_creates_without_proxy() {
    let result = OpenRouterClient::new("key".to_string(), "model".to_string(), None);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_openrouter_client_creates_with_valid_proxy() {
    let result = OpenRouterClient::new(
        "key".to_string(),
        "model".to_string(),
        Some("http://127.0.0.1:8080".to_string()),
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_openrouter_client_invalid_proxy_warns_but_succeeds() {
    let result = OpenRouterClient::new(
        "key".to_string(),
        "model".to_string(),
        Some("not-a-valid-proxy://".to_string()),
    );
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// Gemini client creation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_gemini_client_creates_without_proxy() {
    let result = GeminiClient::new("key".to_string(), "gemini-2.0-flash".to_string(), None);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gemini_client_creates_with_valid_proxy() {
    let result = GeminiClient::new(
        "key".to_string(),
        "gemini-2.0-flash".to_string(),
        Some("http://127.0.0.1:8080".to_string()),
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_gemini_client_invalid_proxy_warns_but_succeeds() {
    let result = GeminiClient::new(
        "key".to_string(),
        "gemini-2.0-flash".to_string(),
        Some("not-a-valid-proxy://".to_string()),
    );
    assert!(result.is_ok());
}

// ---------------------------------------------------------------------------
// ApiClient enum creation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_api_client_from_openrouter() {
    use ghostwriter_lib::api::ApiClient;
    let c = OpenRouterClient::new("key".to_string(), "model".to_string(), None).unwrap();
    let client = ApiClient::OpenRouter(c);
    let err = client.refine_text("Be concise.", "Hello world").await;
    assert!(err.is_err());
    let msg = err.unwrap_err();
    assert!(msg.contains("Network error") || msg.contains("error"));
}

#[tokio::test]
async fn test_api_client_from_gemini() {
    use ghostwriter_lib::api::ApiClient;
    let c = GeminiClient::new("key".to_string(), "gemini-2.0-flash".to_string(), None).unwrap();
    let client = ApiClient::Google(c);
    let err = client.refine_text("Be concise.", "Hello world").await;
    assert!(err.is_err());
    let msg = err.unwrap_err();
    assert!(msg.contains("Network error") || msg.contains("error"));
}
