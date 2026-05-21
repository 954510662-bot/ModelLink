use serde_json::Value;

use crate::errors::{ModelLinkError, Result};

#[derive(Debug, Clone)]
pub struct RequestValidator;

impl RequestValidator {
    pub fn validate_chat_completion_request(request: &Value) -> Result<()> {
        if request.get("model").and_then(|v| v.as_str()).is_none() {
            return Err(ModelLinkError::ValidationError("Missing 'model' field".to_string()));
        }

        let messages = request.get("messages").and_then(|v| v.as_array());
        if messages.is_none() || messages.unwrap().is_empty() {
            return Err(ModelLinkError::ValidationError("Messages array must not be empty".to_string()));
        }

        if let Some(messages_array) = messages {
            for (i, message) in messages_array.iter().enumerate() {
                Self::validate_message(message, i)?;
            }
        }

        if let Some(temperature) = request.get("temperature").and_then(|v| v.as_f64()) {
            if temperature < 0.0 || temperature > 2.0 {
                return Err(ModelLinkError::ValidationError("Temperature must be between 0.0 and 2.0".to_string()));
            }
        }

        if let Some(max_tokens) = request.get("max_tokens").and_then(|v| v.as_u64()) {
            if max_tokens > 1_000_000 {
                return Err(ModelLinkError::ValidationError("max_tokens must be less than 1,000,000".to_string()));
            }
        }

        if let Some(top_p) = request.get("top_p").and_then(|v| v.as_f64()) {
            if top_p < 0.0 || top_p > 1.0 {
                return Err(ModelLinkError::ValidationError("top_p must be between 0.0 and 1.0".to_string()));
            }
        }

        Ok(())
    }

    fn validate_message(message: &Value, index: usize) -> Result<()> {
        let role = message.get("role").and_then(|v| v.as_str());
        if role.is_none() {
            return Err(ModelLinkError::ValidationError(format!(
                "Message at index {} missing 'role' field",
                index
            )));
        }

        let valid_roles = ["user", "assistant", "system", "tool"];
        if !valid_roles.contains(&role.unwrap()) {
            return Err(ModelLinkError::ValidationError(format!(
                "Invalid role '{}' at message index {}. Valid roles: user, assistant, system, tool",
                role.unwrap(),
                index
            )));
        }

        let has_content = message.get("content").is_some();
        let has_function_call = message.get("function_call").is_some();
        let has_tool_calls = message.get("tool_calls").is_some();

        if !has_content && !has_function_call && !has_tool_calls {
            return Err(ModelLinkError::ValidationError(format!(
                "Message at index {} must have 'content', 'function_call', or 'tool_calls'",
                index
            )));
        }

        if let Some(content) = message.get("content").and_then(|v| v.as_str()) {
            if content.chars().count() > 1_000_000 {
                return Err(ModelLinkError::ValidationError(format!(
                    "Message content at index {} exceeds 1,000,000 characters",
                    index
                )));
            }
        }

        Ok(())
    }

    pub fn validate_anthropic_message_request(request: &Value) -> Result<()> {
        let messages = request.get("messages").and_then(|v| v.as_array());
        if messages.is_none() || messages.unwrap().is_empty() {
            return Err(ModelLinkError::ValidationError("Messages array must not be empty".to_string()));
        }

        if let Some(messages_array) = messages {
            for (i, message) in messages_array.iter().enumerate() {
                Self::validate_anthropic_message(message, i)?;
            }
        }

        if let Some(max_tokens) = request.get("max_tokens").and_then(|v| v.as_u64()) {
            if max_tokens > 200_000 {
                return Err(ModelLinkError::ValidationError("max_tokens must be less than 200,000".to_string()));
            }
        }

        if let Some(temperature) = request.get("temperature").and_then(|v| v.as_f64()) {
            if temperature < 0.0 || temperature > 1.0 {
                return Err(ModelLinkError::ValidationError("Temperature must be between 0.0 and 1.0".to_string()));
            }
        }

        Ok(())
    }

    fn validate_anthropic_message(message: &Value, index: usize) -> Result<()> {
        let role = message.get("role").and_then(|v| v.as_str());
        if role.is_none() {
            return Err(ModelLinkError::ValidationError(format!(
                "Message at index {} missing 'role' field",
                index
            )));
        }

        let valid_roles = ["user", "assistant"];
        if !valid_roles.contains(&role.unwrap()) {
            return Err(ModelLinkError::ValidationError(format!(
                "Invalid role '{}' at message index {}. Valid roles: user, assistant",
                role.unwrap(),
                index
            )));
        }

        let content = message.get("content");
        if content.is_none() {
            return Err(ModelLinkError::ValidationError(format!(
                "Message at index {} must have 'content'",
                index
            )));
        }

        Ok(())
    }

    pub fn validate_api_key(api_key: &str) -> Result<()> {
        if api_key.is_empty() {
            return Err(ModelLinkError::ValidationError("API key cannot be empty".to_string()));
        }

        if api_key.len() < 10 {
            return Err(ModelLinkError::ValidationError("API key must be at least 10 characters".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_chat_completion_valid() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ],
            "temperature": 0.7,
            "max_tokens": 100
        });

        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }

    #[test]
    fn test_validate_chat_completion_missing_model() {
        let request = json!({
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ]
        });

        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("model"));
    }

    #[test]
    fn test_validate_chat_completion_empty_messages() {
        let request = json!({
            "model": "gpt-4",
            "messages": []
        });

        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_validate_chat_completion_invalid_role() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "invalid",
                    "content": "Hello"
                }
            ]
        });

        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid role"));
    }

    #[test]
    fn test_validate_chat_completion_temperature_range() {
        let request_too_high = json!({
            "model": "gpt-4",
            "messages": [{ "role": "user", "content": "Hello" }],
            "temperature": 3.0
        });
        assert!(RequestValidator::validate_chat_completion_request(&request_too_high).is_err());

        let request_too_low = json!({
            "model": "gpt-4",
            "messages": [{ "role": "user", "content": "Hello" }],
            "temperature": -0.1
        });
        assert!(RequestValidator::validate_chat_completion_request(&request_too_low).is_err());
    }

    #[test]
    fn test_validate_anthropic_message_valid() {
        let request = json!({
            "model": "claude-3-opus",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ],
            "max_tokens": 1000
        });

        assert!(RequestValidator::validate_anthropic_message_request(&request).is_ok());
    }

    #[test]
    fn test_validate_api_key() {
        assert!(RequestValidator::validate_api_key("sk-1234567890abcdef").is_ok());
        assert!(RequestValidator::validate_api_key("").is_err());
        assert!(RequestValidator::validate_api_key("short").is_err());
    }

    #[test]
    fn test_validate_message_missing_content() {
        let message = json!({
            "role": "user"
        });

        let result = RequestValidator::validate_message(&message, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("content"));
    }

    #[test]
    fn test_validate_anthropic_invalid_role() {
        let request = json!({
            "model": "claude-3-opus",
            "messages": [
                {
                    "role": "system",
                    "content": "Hello"
                }
            ]
        });

        let result = RequestValidator::validate_anthropic_message_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid role"));
    }
}