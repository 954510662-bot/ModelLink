use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIChatRequest {
    pub model: String,
    
    pub messages: Vec<OpenAIMessage>,
    
    #[serde(default)]
    pub temperature: Option<f32>,
    
    #[serde(default)]
    pub max_tokens: Option<u32>,
    
    #[serde(default)]
    pub top_p: Option<f32>,
    
    #[serde(default)]
    pub top_k: Option<u32>,
    
    #[serde(default)]
    pub stream: bool,
    
    #[serde(default)]
    pub functions: Option<Vec<OpenAIFunction>>,
    
    #[serde(default)]
    pub function_call: Option<OpenAIFunctionCall>,
    
    #[serde(default)]
    pub tools: Option<Vec<OpenAITool>>,
    
    #[serde(default)]
    pub tool_choice: Option<OpenAIToolChoice>,
    
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIMessage {
    pub role: String,
    
    pub content: String,
    
    #[serde(default)]
    pub name: Option<String>,
    
    #[serde(default)]
    pub function_call: Option<OpenAIFunctionCall>,
    
    #[serde(default)]
    pub tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIFunction {
    pub name: String,
    
    pub description: Option<String>,
    
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIFunctionCall {
    pub name: String,
    
    #[serde(default)]
    pub arguments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAITool {
    pub type_field: String,
    
    pub function: OpenAIFunction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum OpenAIToolChoice {
    String(String),
    Object(OpenAIToolChoiceObject),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIToolChoiceObject {
    pub type_field: String,
    
    #[serde(default)]
    pub function: Option<OpenAIToolChoiceFunction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIToolChoiceFunction {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIToolCall {
    pub id: String,
    
    pub type_field: String,
    
    pub function: OpenAIFunctionCall,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIChatResponse {
    pub id: String,
    
    pub object: String,
    
    pub created: u64,
    
    pub model: String,
    
    pub choices: Vec<OpenAIChoice>,
    
    #[serde(default)]
    pub usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIChoice {
    pub index: u32,
    
    pub message: Option<OpenAIMessage>,
    
    pub delta: Option<OpenAIDelta>,
    
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIDelta {
    #[serde(default)]
    pub role: Option<String>,
    
    #[serde(default)]
    pub content: Option<String>,
    
    #[serde(default)]
    pub function_call: Option<OpenAIFunctionCall>,
    
    #[serde(default)]
    pub tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    
    pub completion_tokens: u32,
    
    pub total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicMessageRequest {
    pub model: String,
    
    #[serde(default)]
    pub max_tokens: Option<u32>,
    
    #[serde(default)]
    pub temperature: Option<f32>,
    
    #[serde(default)]
    pub top_p: Option<f32>,
    
    #[serde(default)]
    pub top_k: Option<u32>,
    
    #[serde(default)]
    pub stream: bool,
    
    pub messages: Vec<AnthropicMessage>,
    
    #[serde(default)]
    pub system: Option<String>,
    
    #[serde(default)]
    pub tools: Option<Vec<AnthropicTool>>,
    
    #[serde(default)]
    pub tool_choice: Option<AnthropicToolChoice>,
    
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicMessage {
    pub role: String,
    
    #[serde(default)]
    pub content: AnthropicContent,
    
    #[serde(default)]
    pub tool_calls: Option<Vec<AnthropicToolCall>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum AnthropicContent {
    String(String),
    Blocks(Vec<AnthropicContentBlock>),
}

impl Default for AnthropicContent {
    fn default() -> Self {
        AnthropicContent::String(String::new())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicContentBlock {
    pub type_field: String,
    
    pub text: Option<String>,
    
    pub tool_use: Option<AnthropicToolUse>,
    
    pub tool_result: Option<AnthropicToolResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicToolUse {
    pub id: String,
    
    pub name: String,
    
    #[serde(default)]
    pub input: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicToolResult {
    pub tool_use_id: String,
    
    pub content: Option<String>,
    
    #[serde(default)]
    pub is_error: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicTool {
    pub name: String,
    
    pub description: String,
    
    #[serde(default)]
    pub input_schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum AnthropicToolChoice {
    String(String),
    Object(AnthropicToolChoiceObject),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicToolChoiceObject {
    pub type_field: String,
    
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicToolCall {
    pub id: String,
    
    pub name: String,
    
    #[serde(default)]
    pub input: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicMessageResponse {
    pub id: String,
    
    #[serde(rename = "type")]
    pub type_field: String,
    
    pub role: String,
    
    pub content: Vec<AnthropicContentBlock>,
    
    pub model: String,
    
    pub stop_reason: Option<String>,
    
    pub usage: AnthropicUsage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicUsage {
    pub input_tokens: u32,
    
    pub output_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    pub type_field: String,
    
    pub message: Option<AnthropicMessageResponse>,
    
    pub message_start: Option<AnthropicMessageStart>,
    
    pub content_block_start: Option<AnthropicContentBlockStart>,
    
    pub content_block_delta: Option<AnthropicContentBlockDelta>,
    
    pub content_block_stop: Option<AnthropicContentBlockStop>,
    
    pub message_delta: Option<AnthropicMessageDelta>,
    
    pub message_stop: Option<AnthropicMessageStop>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicMessageStart {
    pub message: AnthropicMessageResponse,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicContentBlockStart {
    pub index: u32,
    
    pub content_block: AnthropicContentBlock,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicContentBlockDelta {
    pub index: u32,
    
    pub delta: AnthropicDelta,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicDelta {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicContentBlockStop {
    pub index: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicMessageDelta {
    pub stop_reason: Option<String>,
    
    pub usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnthropicMessageStop {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorDetail {
    pub message: String,
    
    pub type_field: String,
    
    #[serde(default)]
    pub code: Option<String>,
}
