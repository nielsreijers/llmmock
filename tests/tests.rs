use llmmock::LlmMockBuilder;

#[tokio::test]
async fn test_health() {
    let builder = LlmMockBuilder::new();
    let mock = builder.start().await.unwrap();
    let client = reqwest::Client::new();

    // Act: Send a real HTTP request to the assigned port
    let response = client
        .get(&format!("http://localhost:{}/health", mock.port()))
        .send().await
        .unwrap();

    // Assert: Check the results
    assert!(response.status().is_success());
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_models() {
    let builder = LlmMockBuilder::new();
    let builder = builder.with_models("foobar".to_string());
    let mock = builder.start().await.unwrap();
    let client = reqwest::Client::new();

    // Act: Send a real HTTP request to the assigned port
    let response = client
        .get(&format!("http://localhost:{}/v1/models", mock.port()))
        .send().await
        .unwrap();

    // Assert: Check the results
    assert!(response.status().is_success());
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "foobar");
}

#[tokio::test]
async fn test_chat_completions() {
    let builder = LlmMockBuilder::new();
    let builder = builder.with_models("foobar".to_string());
    let mock = builder.start().await.unwrap();
    let client = reqwest::Client::new();

    // Bad request (missing "model") should result in a 400
    let response = client
        .post(&format!("http://localhost:{}/v1/chat/completions", mock.port()))
        .body(r#"{"messages": [{"role": "user", "content": "hello"}]}"#)
        .send().await
        .unwrap();
    assert!(response.status().is_client_error());
    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    let response = client
        .post(&format!("http://localhost:{}/v1/chat/completions", mock.port()))
        .body(r#"{"model": "mocked-model", "messages": [{"role": "user", "content": "hello"}]}"#)
        .send().await
        .unwrap();
    assert!(response.status().is_success());
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
