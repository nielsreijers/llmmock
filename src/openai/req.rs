use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_json::{ Value };

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionRequest {
    #[allow(unused)] pub messages: Vec<ChatCompletionMessageParam>,
    pub model: String,
    #[allow(unused)] pub audio: Option<ChatCompletionAudioParam>,
    #[allow(unused)] pub frequency_penalty: Option<f32>,
    #[allow(unused)] pub function_call: Option<ChatCompletionFunctionCallOption>,
    #[allow(unused)] pub functions: Option<Vec<ChatCompletionFunction>>,
    #[allow(unused)] pub logit_bias: Option<HashMap<u32, i8>>,
    #[allow(unused)] pub log_probs: Option<bool>,
    #[allow(unused)] pub max_completion_tokens: Option<u32>,
    #[deprecated(note = "replaced by max_completion_tokens")]
    #[allow(unused)]
    pub max_tokens: Option<u32>,
    #[allow(unused)] pub metadata: Option<HashMap<String, String>>,
    #[allow(unused)] pub modalities: Option<Vec<Modality>>,
    #[allow(unused)] pub moderation: Option<Moderation>,
    #[allow(unused)] pub n: Option<u8>,
    #[allow(unused)] pub parallel_tool_calls: Option<bool>,
    #[allow(unused)] pub prediction: Option<PredictionContent>,
    #[allow(unused)] pub presence_penalty: Option<f32>,
    #[allow(unused)] pub prompt_cache_key: Option<String>,
    #[allow(unused)] pub prompt_cache_options: Option<PromptCacheOptions>,
    #[deprecated(note = "replaced by prompt_cache_options")]
    #[allow(unused)]
    pub prompt_cache_retention: Option<PromptCacheRetention>,
    #[allow(unused)] pub reasoning_effort: Option<ReasoningEffort>,
    #[allow(unused)] pub response_format: Option<ResponseFormat>,
    #[allow(unused)] pub safety_identifier: Option<String>,
    #[allow(unused)] pub seed: Option<u64>,
    #[allow(unused)] pub service_tier: Option<ServiceTier>,
    #[allow(unused)] pub stop: Option<Stop>,
    #[allow(unused)] pub store: Option<bool>,
    #[allow(unused)] pub stream: Option<bool>,
    #[allow(unused)] pub stream_options: Option<StreamOptions>,
    #[allow(unused)] pub temperature: Option<f32>,
    #[allow(unused)] pub tool_choice: Option<ToolChoice>,
    #[allow(unused)] pub tools: Option<Vec<Tool>>,
    #[allow(unused)] pub top_logprobs: Option<u8>,
    #[allow(unused)] pub top_p: Option<f32>,
    #[allow(unused)] pub user: Option<String>,
    #[allow(unused)] pub verbosity: Option<Verbosity>,
    #[allow(unused)] pub web_search_options: Option<WebSearchOptions>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "role")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionMessageParam {
    #[allow(unused)] Developer {
        content: ChatCompletionContentTextOnly,
        name: Option<String>,
    },
    #[allow(unused)] System {
        content: ChatCompletionContentTextOnly,
        name: Option<String>,
    },
    #[allow(unused)] User {
        content: ChatCompletionContent,
        name: Option<String>,
    },
    #[allow(unused)] Assistant {
        audio: Option<Audio>,
        content: Option<ChatCompletionContentTextOrRefusal>,
        #[deprecated(note = "replaced by tool_calls")]
        function_call: Option<FunctionCall>,
        name: Option<String>,
        refusal: Option<String>,
        tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
    },
    #[allow(unused)] Tool {
        content: ChatCompletionContentTextOnly,
        tool_call_id: String,
    },
    #[allow(unused)] Function {
        content: Option<String>,
        name: String,
    },
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
    #[allow(unused)] Text {
        text: String,
        prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
    },
    #[allow(unused)] ImageUrl {
        image_url: ImageUrl,
        prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
    },
    #[allow(unused)] InputAudio {
        input_audio: InputAudio,
        prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
    },
    #[allow(unused)] File {
        file: File,
        prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
    },
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
pub(crate) struct File {
    #[allow(unused)] pub file_data: Option<String>,
    #[allow(unused)] pub file_id: Option<String>,
    #[allow(unused)] pub filename: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Audio {
    #[allow(unused)] pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionContentPartTextOrRefusal {
    #[allow(unused)] Text {
        text: String,
        prompt_cache_breakpoint: Option<ChatCompletionPromptCacheBreakpoint>,
    },
    #[allow(unused)] Refusal {
        refusal: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatCompletionContentTextOrRefusal {
    #[allow(unused)] Text(String),
    #[allow(unused)] Structured(Vec<ChatCompletionContentPartTextOrRefusal>),
}
#[derive(Debug, Deserialize)]
pub(crate) struct FunctionCall {
    #[allow(unused)] pub arguments: String,
    #[allow(unused)] pub name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CustomCall {
    #[allow(unused)] pub input: String,
    #[allow(unused)] pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionMessageToolCall {
    #[allow(unused)] Function {
        id: String,
        function: FunctionCall,
    },
    #[allow(unused)] Custom {
        id: String,
        custom: CustomCall,
    },
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
pub(crate) enum ChatCompletionAudioParamFormat {
    Wav,
    Aac,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionAudioParamVoiceBuiltin {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Sage,
    Shimmer,
    Verse,
    Marin,
    Cedar,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ChatCompletionAudioParamVoice {
    #[allow(unused)] Builtin(ChatCompletionAudioParamVoiceBuiltin),
    #[allow(unused)] Custom {
        id: String,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionAudioParam {
    #[allow(unused)] format: ChatCompletionAudioParamFormat,
    #[allow(unused)] voice: ChatCompletionAudioParamVoice,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionFunctionCallOptionBuiltin {
    #[allow(unused)] None,
    #[allow(unused)] Auto,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ChatCompletionFunctionCallOption {
    #[allow(unused)] Builtin(ChatCompletionFunctionCallOptionBuiltin),
    #[allow(unused)] Name {
        name: String,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionFunction {
    #[allow(unused)] name: String,
    #[allow(unused)] description: Option<String>,
    #[allow(unused)] parameters: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Modality {
    #[allow(unused)] Text,
    #[allow(unused)] Audio,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ModerationPolicyMode {
    #[allow(unused)] Score,
    #[allow(unused)] Block,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ModerationPolicyPart {
    #[allow(unused)] mode: ModerationPolicyMode,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ModerationPolicy {
    #[allow(unused)] input: Option<ModerationPolicyPart>,
    #[allow(unused)] output: Option<ModerationPolicyPart>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Moderation {
    #[allow(unused)] model: String,
    #[allow(unused)] policy: Option<ModerationPolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PredictionContentType {
    Content,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PredictionContent {
    #[allow(unused)] content: ChatCompletionContentTextOnly,
    #[allow(unused)] r#type: PredictionContentType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromptCacheMode {
    Implicit,
    Explicit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromptCacheTtl {
    #[serde(rename = "30m")]
    ThirtyMinutes,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PromptCacheOptions {
    #[allow(unused)] mode: Option<PromptCacheMode>,
    #[allow(unused)] ttl: Option<PromptCacheTtl>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PromptCacheRetention {
    InMemory,
    #[serde(rename = "24h")]
    TwentyFourHours,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JsonSchema {
    #[allow(unused)] name: String,
    #[allow(unused)] description: Option<String>,
    #[allow(unused)] schema: Option<Value>,
    #[allow(unused)] strict: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum ResponseFormat {
    #[allow(unused)] Text,
    #[allow(unused)] JsonSchema {
        json_schema: JsonSchema,
    },
    #[allow(unused)] JsonObject,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    Fast,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub(crate) enum Stop {
    #[allow(unused)] String(String),
    #[allow(unused)] List(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub(crate) struct StreamOptions {
    #[allow(unused)] include_obfuscation: Option<bool>,
    #[allow(unused)] include_usage: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ToolChoiceMode {
    #[allow(unused)] None,
    #[allow(unused)] Auto,
    #[allow(unused)] Required,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AllowedToolsMode {
    #[allow(unused)] Auto,
    #[allow(unused)] Required,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum ToolChoiceComplex {
    #[allow(unused)] AllowedTools {
        mode: AllowedToolsMode,
        tools: Vec<Value>,
    },
    #[allow(unused)] Function {
        function: String,
    },
    #[allow(unused)] Custom {
        custom: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub(crate) enum ToolChoice {
    #[allow(unused)] Mode(ToolChoiceMode),
    #[allow(unused)] Complex(ToolChoiceComplex),
}

#[derive(Debug, Deserialize)]
pub(crate) struct FunctionTool {
    #[allow(unused)] name: String,
    #[allow(unused)] description: Option<String>,
    #[allow(unused)] parameters: Option<Value>,
    #[allow(unused)] strict: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CustomToolFormatGrammarSyntax {
    #[allow(unused)] Lark,
    #[allow(unused)] Regex,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CustomToolFormatGrammar {
    #[allow(unused)] pub definition: String,
    #[allow(unused)] pub syntax: CustomToolFormatGrammarSyntax,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum CustomToolFormat {
    #[allow(unused)] Text,
    #[allow(unused)] Grammar {
        grammar: CustomToolFormatGrammar,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct CustomTool {
    #[allow(unused)] name: String,
    #[allow(unused)] description: Option<String>,
    #[allow(unused)] format: Option<CustomToolFormat>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(crate) enum Tool {
    #[allow(unused)] Function {
        function: FunctionTool,
    },
    #[allow(unused)] Custom {
        custom: CustomTool,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Verbosity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WebSearchContextSize {
    Low,
    Medium,
    High,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WebSearchOptions {
    #[allow(unused)] search_context_size: Option<WebSearchContextSize>,
    #[allow(unused)] user_location: Option<Value>,
}
