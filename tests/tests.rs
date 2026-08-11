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
            assert!(status.is_success(), "REQ: {req}\nRESP: {status}: {body}");
            assert_eq!(status, reqwest::StatusCode::OK, "REQ: {req}\nRESP: {status}: {body}");
        } else {
            assert!(status.is_client_error(), "REQ: {req}\nRESP: {status}: {body}");
            assert_eq!(
                status,
                reqwest::StatusCode::BAD_REQUEST,
                "REQ: {req}\nRESP: {status}: {body}"
            );
        }
    };
    let test_ok = async |req| test(req, true).await;
    let test_nok = async |req| test(req, false).await;

    // Ok request
    test_ok(
        r#"{
        "model": "mocked-model",
        "messages": [
            {"role": "developer", "content": "hello world"},
            {"role": "developer", "name": "myname", "content": [
                {"type": "text", "text": "hello world", "prompt_cache_breakpoint": {"mode": "explicit"}},
                {"type": "text", "text": "hello world"}
            ]},
            {"role": "system", "content": "hello world"},
            {"role": "system", "name": "myname", "content": [
                {"type": "text", "text": "hello world"},
                {"type": "text", "text": "hello world"}
            ]},
            {"role": "user", "content": "hello world"},
            {"role": "user", "name": "myname", "content": [
                {"type": "text", "text": "hello world"},
                {"type": "text", "text": "hello world"},
                {"type": "image_url", "image_url": {"url": "hello world"}},
                {"type": "image_url", "image_url": {"url": "hello world", "detail": "low"}},
                {"type": "input_audio", "input_audio": {"data": "hello world", "format": "wav"}},
                {"type": "file", "file": {"file_data": "hello world", "file_id": "hello world", "filename": "hello world"}},
                {"type": "file", "file": {}}
            ]},
            {"role": "assistant", "content": "hello world"},
            {"role": "assistant", "content": "hello world", "audio": {"id": "hello world"}},
            {"role": "assistant", "name": "myname", "content": [
                {"type": "text", "text": "hello world"}
            ]},
            {"role": "assistant", "name": "myname", "content": [
                {"type": "refusal", "refusal": "hello world"}
            ]},
            {"role": "assistant", "function_call": {"arguments": "hello world", "name": "hello world"}},
            {"role": "assistant", "refusal": "hello world"},
            {"role": "assistant", "tool_calls": [
                {"id": "hello world", "type": "function", "function": {"arguments": "hello world", "name": "hello world"}},
                {"id": "hello world", "type": "custom", "custom": {"input": "hello world", "name": "hello world"}}
            ]},
            {"role": "tool", "tool_call_id": "hello world", "content": "hello world"},
            {"role": "tool", "tool_call_id": "hello world", "content": [
                {"type": "text", "text": "hello world"},
                {"type": "text", "text": "hello world"}
            ]},
            {"role": "function", "name": "hello world"},
            {"role": "function", "name": "hello world", "content": "hello world"}
        ]}"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "audio": {"format": "wav", "voice": "alloy"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "audio": {"format": "wav", "voice": {"id": "hello world"}}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "function_call": "auto"
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "function_call": {"name": "hello world"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "functions": [
                {"name": "hello world"},
                {"name": "hello world", "description": "hello world"},
                {"name": "hello world", "parameters": {"hello world": "hello world"}}
            ]
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "metadata": {
                "hello world": "hello world"
            }
        }"#
    ).await;

    test_nok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "metadata": {
                "hello world": 1
            }
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "logit_bias": {"12345": -100, "654321": 100}
        }"#
    ).await;

    test_nok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "logit_bias": {"hello world": 100}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "moderation": {"model": "hello world"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "moderation": {"model": "hello world", "policy": {"input": {"mode": "score"}}}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "moderation": {"model": "hello world", "policy": {"output": {"mode": "block"}}}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prediction": {"content": "hello world", "type": "content"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prediction": {"content": [{"type": "text", "text": "hello world"}], "type": "content"}
        }"#
    ).await;

    test_nok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prediction": {"content": [{"type": "text", "text": "hello world"}], "type": "text"}
        }"#
    ).await;

    test_nok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prediction": {"content": [{"type": "image_url", "image_url": {"url": "hello world"}}], "type": "content"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prompt_cache_options": {"mode": "implicit"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prompt_cache_options": {"ttl": "30m"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prompt_cache_retention": "in_memory"
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "prompt_cache_retention": "24h"
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "response_format": {"type": "text"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "response_format": {"type": "json_schema", "json_schema": {"name": "hello world"}}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "response_format": { "type": "json_schema", "json_schema": {        
                "name": "hello world",
                "description": "hello world",
                "schema": {"hello world": "hello world"},
                "strict": false
            }}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "stop": "hello world"
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "stop": ["hello world", "hello world"]
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "response_format": { "type": "json_schema", "json_schema": {        
                "name": "hello world",
                "description": "hello world",
                "schema": {"hello world": "hello world"},
                "strict": false
            }}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "tool_choice": "required"
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "tool_choice": {"type": "function", "function": "hello world"}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "tool_choice": {"type": "allowed_tools", "mode": "auto", "tools": [{"hello world": "hello world"}]}
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "tools": [
                {"type": "function", "function": {"name": "hello world"}},
                {"type": "function", "function": {"name": "hello world", "description": "hello world", "parameters": {"hello world": "hello world"}, "strict": true}},
                {"type": "custom", "custom": {"name": "hello world"}},
                {"type": "custom", "custom": {"name": "hello world", "description": "hello world", "format": {"type": "text"}}},
                {"type": "custom", "custom": {"name": "hello world", "description": "hello world", "format": {"type": "grammar", "grammar": {"definition": "hello world", "syntax": "lark"}}}}
            ]
        }"#
    ).await;

    test_ok(
        r#"{
            "model": "mocked-model",
            "messages": [{"role": "developer", "content": "hello world"}],
            "frequency_penalty": 0.0,
            "log_probs": true,
            "max_completion_tokens": 654321,
            "max_tokens": 654321,
            "modalities": ["text", "audio"],
            "n": 128,
            "parallel_tool_calls": true,
            "presence_penalty": 2.0,
            "prompt_cache_key": "hello world",
            "reasoning_effort": "medium",
            "safety_identifier": "hello world",
            "seed": 9223372036854776000,
            "store": true,
            "stream": true,
            "stream_options": {"include_obfuscation": true, "include_usage": true},
            "temperature": 0.2,
            "top_logprobs": 2,
            "top_p": 0.2,
            "user": "hello world",
            "verbosity": "low",
            "web_search_options": {"search_context_size": "medium", "user_location": {"hello world": "hello world"}}
        }"#
    ).await;

    // Bad request (missing "model") should result in a 400
    test_nok(r#"{"messages": [{"role": "user", "content": "hello"}]}"#).await;
}
