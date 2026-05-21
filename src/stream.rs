use axum::{
    body::Body,
    http::HeaderMap,
    response::Response,
};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
    config::Config,
    errors::{ModelLinkError, Result},
    models::{AnthropicStreamEvent, OpenAIChoice, OpenAIDelta, OpenAIChatResponse},
    utils::convert_headers,
};

pub async fn forward_streaming_request(
    _config: &Config,
    _model_name: &str,
    request_body: Value,
    upstream_url: String,
    headers: HeaderMap,
) -> Result<Response> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(&upstream_url)
        .headers(convert_headers(&headers))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| ModelLinkError::NetworkError(e.to_string()))?;
    
    if !response.status().is_success() {
        let body = response.text().await.map_err(|e| ModelLinkError::NetworkError(e.to_string()))?;
        return Err(ModelLinkError::ProtocolError(format!(
            "Upstream returned error: {}",
            body
        )));
    }
    
    let body = response.bytes().await.map_err(|e| ModelLinkError::NetworkError(e.to_string()))?;
    Ok(Response::new(Body::from(body)))
}

pub fn transform_anthropic_to_openai_stream(event: &str) -> Result<String> {
    let anthropic_event: AnthropicStreamEvent = serde_json::from_str(event)
        .map_err(|e| ModelLinkError::TransformError(format!("Failed to parse Anthropic event: {}", e)))?;
    
    let openai_event = match anthropic_event.type_field.as_str() {
        "message_start" => {
            if let Some(start) = anthropic_event.message_start {
                let delta = OpenAIDelta {
                    role: Some("assistant".to_string()),
                    content: None,
                    function_call: None,
                    tool_calls: None,
                };
                
                json!({
                    "id": start.message.id,
                    "object": "chat.completion.chunk",
                    "created": 0,
                    "model": start.message.model,
                    "choices": [{
                        "index": 0,
                        "delta": delta,
                        "finish_reason": null
                    }]
                })
            } else {
                return Ok(String::new());
            }
        }
        "content_block_delta" => {
            if let Some(delta) = anthropic_event.content_block_delta {
                let content = delta.delta.text.clone().unwrap_or_default();
                
                let delta = OpenAIDelta {
                    role: None,
                    content: Some(content),
                    function_call: None,
                    tool_calls: None,
                };
                
                json!({
                    "id": "chatcmpl-stream",
                    "object": "chat.completion.chunk",
                    "created": 0,
                    "model": "anthropic-model",
                    "choices": [{
                        "index": 0,
                        "delta": delta,
                        "finish_reason": null
                    }]
                })
            } else {
                return Ok(String::new());
            }
        }
        "message_stop" => {
            json!({
                "id": "chatcmpl-stream",
                "object": "chat.completion.chunk",
                "created": 0,
                "model": "anthropic-model",
                "choices": [{
                    "index": 0,
                    "delta": {},
                    "finish_reason": "stop"
                }]
            })
        }
        _ => {
            return Ok(String::new());
        }
    };
    
    serde_json::to_string(&openai_event)
        .map_err(|e| ModelLinkError::TransformError(format!("Failed to serialize OpenAI event: {}", e)))
}

pub fn transform_openai_request(request: Value, capabilities: &HashMap<String, bool>) -> Value {
    let mut request = request;
    
    let max_temp_from_request = request.get("max_temperature")
        .and_then(|v| v.as_f64());
    
    if !capabilities.get("supports_temperature").copied().unwrap_or(true) {
        request.as_object_mut().map(|obj| obj.remove("temperature"));
    } else if let Some(max_temp) = max_temp_from_request {
        if let Some(temperature) = request.get_mut("temperature") {
            if let Some(temp) = temperature.as_f64() {
                if temp > max_temp {
                    *temperature = json!(max_temp);
                }
            }
        }
    }
    
    if let Some(_top_p) = request.get("top_p") {
        if !capabilities.get("supports_top_p").copied().unwrap_or(true) {
            request.as_object_mut().map(|obj| obj.remove("top_p"));
        }
    }
    
    if let Some(_top_k) = request.get("top_k") {
        if !capabilities.get("supports_top_k").copied().unwrap_or(false) {
            request.as_object_mut().map(|obj| obj.remove("top_k"));
        }
    }
    
    request
}

pub fn build_openai_response(content: &str, model: &str) -> Value {
    let response = OpenAIChatResponse {
        id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp() as u64,
        model: model.to_string(),
        choices: vec![OpenAIChoice {
            index: 0,
            message: Some(crate::models::OpenAIMessage {
                role: "assistant".to_string(),
                content: content.to_string(),
                name: None,
                function_call: None,
                tool_calls: None,
            }),
            delta: None,
            finish_reason: Some("stop".to_string()),
        }],
        usage: Some(crate::models::OpenAIUsage {
            prompt_tokens: 0,
            completion_tokens: content.len() as u32 / 4,
            total_tokens: content.len() as u32 / 4,
        }),
    };
    
    serde_json::to_value(response).unwrap_or_default()
}

pub fn parse_sse_events(bytes: &[u8]) -> Result<Vec<String>> {
    let text = String::from_utf8_lossy(bytes);
    let mut events = Vec::new();
    
    for line in text.lines() {
        if line.starts_with("data: ") {
            let data = line.trim_start_matches("data: ");
            if !data.is_empty() && data != "[DONE]" {
                events.push(data.to_string());
            }
        }
    }
    
    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transform_openai_request_remove_top_k() {
        let mut capabilities = HashMap::new();
        capabilities.insert("supports_top_k".to_string(), false);
        capabilities.insert("supports_temperature".to_string(), true);
        capabilities.insert("supports_top_p".to_string(), true);
        
        let request = json!({
            "model": "test-model",
            "messages": [],
            "temperature": 0.7,
            "top_k": 50
        });
        
        let result = transform_openai_request(request, &capabilities);
        assert!(result.get("top_k").is_none());
        assert_eq!(result["temperature"], 0.7);
    }
    
    #[test]
    fn test_transform_anthropic_to_openai_stream() {
        let anthropic_event = json!({
            "type": "content_block_delta",
            "content_block_delta": {
                "index": 0,
                "delta": {
                    "text": "Hello"
                }
            }
        });
        
        let result = transform_anthropic_to_openai_stream(&anthropic_event.to_string()).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["object"], "chat.completion.chunk");
        assert_eq!(parsed["choices"][0]["delta"]["content"], "Hello");
    }
    
    #[test]
    fn test_parse_sse_events_basic() {
        let bytes = b"data: {\"type\":\"content_block_delta\",\"delta\":{\"text\":\"Hello\"}}\n\n";
        let events = parse_sse_events(bytes).unwrap();
        
        assert_eq!(events.len(), 1);
        assert!(events[0].contains("Hello"));
    }
    
    #[test]
    fn test_parse_sse_events_multiple_events() {
        let bytes = b"data: {\"type\":\"a\"}\ndata: {\"type\":\"b\"}\ndata: {\"type\":\"c\"}\n";
        let events = parse_sse_events(bytes).unwrap();
        
        assert_eq!(events.len(), 3);
    }
    
    #[test]
    fn test_parse_sse_events_with_done() {
        let bytes = b"data: {\"type\":\"a\"}\ndata: [DONE]\n";
        let events = parse_sse_events(bytes).unwrap();
        
        assert_eq!(events.len(), 1);
        assert!(!events[0].contains("DONE"));
    }
    
    #[test]
    fn test_parse_sse_events_empty_data() {
        let bytes = b"data: \ndata: \n";
        let events = parse_sse_events(bytes).unwrap();
        
        assert_eq!(events.len(), 0);
    }
    
    #[test]
    fn test_parse_sse_events_no_prefix() {
        let bytes = b"{\"type\":\"not-event\"}\n";
        let events = parse_sse_events(bytes).unwrap();
        
        assert_eq!(events.len(), 0);
    }
    
    #[test]
    fn test_parse_sse_events_with_newlines() {
        let bytes = b"data: {\"msg\": \"line1\\nline2\"}\n";
        let events = parse_sse_events(bytes).unwrap();
        
        assert_eq!(events.len(), 1);
        assert!(events[0].contains("line1"));
    }
    
    #[test]
    fn test_transform_openai_request_clamp_temperature() {
        let mut capabilities = HashMap::new();
        capabilities.insert("supports_temperature".to_string(), true);
        capabilities.insert("supports_top_p".to_string(), true);
        capabilities.insert("supports_top_k".to_string(), true);
        
        let request = json!({
            "model": "claude-3",
            "messages": [],
            "temperature": 1.5,
            "max_temperature": 1.0
        });
        
        let result = transform_openai_request(request, &capabilities);
        
        assert_eq!(result["temperature"], 1.0);
    }
    
    #[test]
    fn test_transform_openai_request_remove_unsupported_top_p() {
        let mut capabilities = HashMap::new();
        capabilities.insert("supports_temperature".to_string(), true);
        capabilities.insert("supports_top_p".to_string(), false);
        capabilities.insert("supports_top_k".to_string(), true);
        
        let request = json!({
            "model": "claude",
            "messages": [],
            "top_p": 0.9
        });
        
        let result = transform_openai_request(request, &capabilities);
        
        assert!(result.get("top_p").is_none());
    }
    
    #[test]
    fn test_build_openai_response() {
        let content = "This is a test response.";
        let model = "gpt-4";
        
        let result = build_openai_response(content, model);
        
        assert!(result["id"].as_str().unwrap().starts_with("chatcmpl-"));
        assert_eq!(result["object"], "chat.completion");
        assert_eq!(result["model"], model);
        assert_eq!(result["choices"][0]["message"]["content"], content);
        assert_eq!(result["choices"][0]["finish_reason"], "stop");
    }
    
    #[test]
    fn test_transform_anthropic_message_start() {
        let anthropic_event = json!({
            "type": "message_start",
            "message_start": {
                "message": {
                    "id": "msg_123",
                    "type": "message",
                    "role": "assistant",
                    "content": [],
                    "model": "claude-3-opus",
                    "usage": {
                        "input_tokens": 100,
                        "output_tokens": 0
                    }
                }
            }
        });
        
        let result = transform_anthropic_to_openai_stream(&anthropic_event.to_string()).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["id"], "msg_123");
        assert_eq!(parsed["choices"][0]["delta"]["role"], "assistant");
    }
    
    #[test]
    fn test_transform_anthropic_message_stop() {
        let anthropic_event = json!({
            "type": "message_stop",
            "message_stop": {}
        });
        
        let result = transform_anthropic_to_openai_stream(&anthropic_event.to_string()).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["choices"][0]["finish_reason"], "stop");
    }
    
    #[test]
    fn test_transform_anthropic_unknown_event() {
        let anthropic_event = json!({
            "type": "ping"
        });
        
        let result = transform_anthropic_to_openai_stream(&anthropic_event.to_string()).unwrap();
        
        assert!(result.is_empty());
    }
}
