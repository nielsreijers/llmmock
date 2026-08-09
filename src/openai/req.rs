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
    #[allow(unused)] User(ChatCompletionUserMessageParam),
    #[allow(unused)] Assistant(ChatCompletionAssistantMessageParam),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatCompletionContentTextOnly {
    #[allow(unused)] Text(String),
    #[allow(unused)] Structured(Vec<ChatCompletionContentPartTextOnly>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionContentPartTextOnlyType {
    Text,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionContentPartTextOnly {
    #[allow(unused)] pub text: String,
    #[allow(unused)] pub r#type: ChatCompletionContentPartTextOnlyType,
    #[allow(unused)] pub prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatCompletionContent {
    #[allow(unused)] Text(String),
    #[allow(unused)] Structured(Vec<ChatCompletionContentPart>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionContentPart {
    #[allow(unused)] Text(ChatCompletionContentPartText),
    #[allow(unused)] ImageUrl(ChatCompletionContentPartImage),
    #[allow(unused)] InputAudio(ChatCompletionContentPartInputAudio),
    #[allow(unused)] File(FileContentPart),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionContentPartText {
    #[allow(unused)] pub text: String,
    #[allow(unused)] pub prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ImageUrl {
    #[allow(unused)] pub url: String,
    #[allow(unused)] pub detail: Option<ImageUrlDetail>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ImageUrlDetail {
    Auto,
    Low,
    High,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionContentPartImage {
    #[allow(unused)] pub image_url: ImageUrl,
    #[allow(unused)] pub prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InputAudio {
    #[allow(unused)] pub data: String,
    #[allow(unused)] pub format: InputAudioFormat,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum InputAudioFormat {
    Wav,
    Mp3,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionContentPartInputAudio {
    #[allow(unused)] pub input_audio: InputAudio,
    #[allow(unused)] pub prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct File {
    #[allow(unused)] pub file_data: Option<String>,
    #[allow(unused)] pub file_id: Option<String>,
    #[allow(unused)] pub filename: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FileContentPart {
    #[allow(unused)] pub file: File,
    #[allow(unused)] pub prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionContentPartRefusal {
    #[allow(unused)] pub refusal: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Audio {
    #[allow(unused)] pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionContentPartTextOrRefusal {
    #[allow(unused)] Text(ChatCompletionContentPartText),
    #[allow(unused)] Refusal(ChatCompletionContentPartRefusal),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatCompletionContentTextOrRefusal {
    #[allow(unused)] Text(String),
    #[allow(unused)] Structured(Vec<ChatCompletionContentPartTextOrRefusal>),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionDeveloperMessageParam {
    #[allow(unused)] pub content: ChatCompletionContentTextOnly,
    #[allow(unused)] pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionSystemMessageParam {
    #[allow(unused)] pub content: ChatCompletionContentTextOnly,
    #[allow(unused)] pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionUserMessageParam {
    #[allow(unused)] pub content: ChatCompletionContent,
    #[allow(unused)] pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FunctionCall {
    #[allow(unused)] pub arguments: String,
    #[allow(unused)] pub name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FunctionToolCall {
    #[allow(unused)] pub id: String,
    #[allow(unused)] pub function: FunctionCall,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CustomCall {
    #[allow(unused)] pub input: String,
    #[allow(unused)] pub name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CustomToolCall {
    #[allow(unused)] pub id: String,
    #[allow(unused)] pub custom: CustomCall,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionMessageToolCall {
    #[allow(unused)] Function(FunctionToolCall),
    #[allow(unused)] Custom(CustomToolCall),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionAssistantMessageParam {
    #[allow(unused)] pub audio: Option<Audio>,
    #[allow(unused)] pub content: Option<ChatCompletionContentTextOrRefusal>,
    #[deprecated(note = "replaced by tool_calls")]
    #[allow(unused)]
    pub function_call: Option<FunctionCall>,
    #[allow(unused)] pub name: Option<String>,
    #[allow(unused)] pub refusal: Option<String>,
    #[allow(unused)] pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
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
