use serde_derive::Deserialize;

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
