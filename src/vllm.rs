use crate::openai;

use super::LlmMockState;
use axum::{
    body::Bytes,
    extract::State,
    http::{ HeaderMap, StatusCode },
    Json,
    response::{ IntoResponse, Response },
};
use serde_json::json;
use std::{ sync::Arc };

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    #[allow(unused)] Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::BadRequest(message) =>
                (
                    StatusCode::BAD_REQUEST,
                    openai::ErrorResponse {
                        error: openai::ErrorBody {
                            message,
                            r#type: "invalid_request_error".into(),
                            param: None,
                            code: None,
                        },
                    },
                ),

            ApiError::Internal(message) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    openai::ErrorResponse {
                        error: openai::ErrorBody {
                            message,
                            r#type: "server_error".into(),
                            param: None,
                            code: None,
                        },
                    },
                ),
        };
        (status, Json(body)).into_response()
    }
}

pub(crate) async fn handle_health() -> &'static str {
    ""
}

pub(crate) async fn handle_models(State(state): State<Arc<LlmMockState>>) -> String {
    state.models.clone()
}

pub(crate) async fn handle_chat_completions(
    State(_state): State<Arc<LlmMockState>>,
    _: HeaderMap,
    body: Bytes
) -> Result<Response, ApiError> {
    let body = String::from_utf8_lossy(&body);

    let request: openai::ChatCompletionRequest = serde_json
        ::from_str(&body)
        .map_err(|err| ApiError::BadRequest(err.to_string()))?;

    let response =
        json!({
        "id": "chatcmpl-mock-123",
        "object": "chat.completion",
        "created": 1750000000u64,
        "model": request.model,
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello! This is a mock response."
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 7,
            "total_tokens": 17
        }
    });

    Ok(Json(response).into_response())
}
