use serde_json::json;

/// Create test payload for different endpoints
pub fn create_test_payload(
    endpoint: &str,
    streaming: bool,
    request_num: usize,
) -> serde_json::Value {
    match endpoint {
        "/v1/chat/completions" => json!({
            "model": "test-model",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": format!("Test request {}", request_num)}
            ],
            "stream": streaming,
            "max_tokens": 10
        }),
        "/v1/completions" => json!({
            "model": "test-model",
            "prompt": format!("Complete this: Test request {}", request_num),
            "stream": streaming,
            "max_tokens": 10
        }),
        _ => json!({
            "text": format!("Load test request {}", request_num),
            "stream": streaming,
            "max_new_tokens": 10
        }),
    }
}

/// Parse endpoint name to URL path
pub fn parse_endpoint(endpoint_name: &str) -> &'static str {
    match endpoint_name {
        "chat" => "/v1/chat/completions",
        "completions" => "/v1/completions",
        _ => "/generate",
    }
}
