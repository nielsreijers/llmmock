use serde_derive::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionRequest {
    pub model: String,
    #[allow(unused)] pub messages: Vec<ChatMessage>,
    #[allow(unused)] pub stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatMessage {
    #[allow(unused)] pub role: String,
    #[allow(unused)] pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub(crate) struct ErrorBody {
    pub message: String,
    pub r#type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}
