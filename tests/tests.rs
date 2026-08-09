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

    let test = async |req: &'static str, ok: bool| {
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("http://localhost:{}/v1/chat/completions", mock.port()))
            .body(req)
            .send().await
            .unwrap();
        let status = response.status();
        let body = match response.text().await {
            Ok(body) => body,
            Err(err) => err.to_string(),
        };

        if ok {
            assert!(status.is_success(), "{status}: {body}");
            assert_eq!(status, reqwest::StatusCode::OK, "{status}: {body}");
        } else {
            assert!(status.is_client_error(), "{status}: {body}");
            assert_eq!(status, reqwest::StatusCode::BAD_REQUEST, "{status}: {body}");
        }
    };
    let test_ok = async |req| test(req, true).await;
    let test_nok = async |req| test(req, false).await;

    // Ok request
    test_ok(
        r#"{
        "model": "mocked-model",
        "messages": [
            {"role": "developer", "content": "hello 1"},
            {"role": "developer", "name": "myname", "content": [
                {"type": "text", "text": "hello 3", "prompt_cache_breakpoint": {"mode": "explicit"}},
                {"type": "text", "text": "hello 4"}
            ]},
            {"role": "system", "content": "hello 5"},
            {"role": "system", "name": "myname", "content": [
                {"type": "text", "text": "hello 6"},
                {"type": "text", "text": "hello 7"}
            ]},
            {"role": "user", "content": "hello 8"},
            {"role": "user", "name": "myname", "content": [
                {"type": "text", "text": "hello 9"},
                {"type": "text", "text": "hello 10"},
                {"type": "image_url", "image_url": {"url": "hello 11"}},
                {"type": "image_url", "image_url": {"url": "hello 12", "detail": "low"}},
                {"type": "input_audio", "input_audio": {"data": "hello 13", "format": "wav"}},
                {"type": "file", "file": {"file_data": "hello 14", "file_id": "hello 15", "filename": "hello 16"}},
                {"type": "file", "file": {}}
            ]}
        ]}"#
    ).await;

    // Bad request (missing "model") should result in a 400
    test_nok(r#"{"messages": [{"role": "user", "content": "hello"}]}"#).await;
}
