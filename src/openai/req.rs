use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionRequest {
    #[allow(unused)] pub messages: Vec<ChatCompletionMessageParam>,
    pub model: String,
    #[allow(unused)] pub audio: Option<ChatCompletionAudioParam>,
    #[allow(unused)] pub frequency_penalty: Option<f32>,
    #[allow(unused)] pub function_call: Option<ChatCompletionFunctionCallOption>,
    #[allow(unused)] pub functions: Option<Vec<ChatCompletionFunction>>,
    #[allow(unused)] pub stream: Option<bool>,
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
