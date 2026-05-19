use ghostwriter_lib::api::OpenRouterClient;

#[tokio::test]
async fn test_client_creates_without_proxy() {
    let result = OpenRouterClient::new("key".to_string(), "model".to_string(), None);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_creates_with_valid_proxy() {
    let result = OpenRouterClient::new(
        "key".to_string(),
        "model".to_string(),
        Some("http://127.0.0.1:8080".to_string()),
    );
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_invalid_proxy_warns_but_succeeds() {
    let result = OpenRouterClient::new(
        "key".to_string(),
        "model".to_string(),
        Some("not-a-valid-proxy://".to_string()),
    );
    assert!(result.is_ok());
}
