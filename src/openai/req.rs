use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionRequest {
    #[allow(unused)] pub messages: Vec<ChatCompletionMessageParam>,
    pub model: String,
    #[allow(unused)] pub stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "role")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionMessageParam {
    #[allow(unused)] Developer(ChatCompletionDeveloperMessageParam),
    #[allow(unused)] System(ChatCompletionSystemMessageParam),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatCompletionMessageContentText {
    #[allow(unused)] Text(String),
    #[allow(unused)] Structured(Vec<ChatCompletionMessageContentPartText>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionPromptCacheBreakpointMode {
    Explicit,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionPromptCacheBreakpoint {
    #[allow(unused)] mode: ChatCompletionPromptCacheBreakpointMode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionMessageContentPartTextType {
    Text,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionMessageContentPartText {
    #[allow(unused)] pub text: String,
    #[allow(unused)] pub r#type: ChatCompletionMessageContentPartTextType,
    #[allow(unused)] pub prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionDeveloperMessageParam {
    #[allow(unused)] pub content: ChatCompletionMessageContentText,
    #[allow(unused)] pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionSystemMessageParam {
    #[allow(unused)] pub content: ChatCompletionMessageContentText,
    #[allow(unused)] pub name: Option<String>,
}
