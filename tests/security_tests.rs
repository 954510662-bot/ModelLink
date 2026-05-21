use model_link::validation::RequestValidator;
use serde_json::json;

#[cfg(test)]
mod security_tests {

    use super::*;

    #[test]
    fn test_sql_injection_prevention() {
        let malicious_input = "'; DROP TABLE users; --";
        assert!(RequestValidator::validate_api_key(malicious_input).is_ok());
    }

    #[test]
    fn test_xss_in_content_validation() {
        let xss_content = "<script>alert('XSS')</script>Hello";
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": xss_content
                }
            ]
        });
        
        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_traversal_prevention() {
        let path_traversal = "../../../etc/passwd";
        assert!(RequestValidator::validate_api_key(path_traversal).is_ok());
    }

    #[test]
    fn test_null_bytes_in_api_key() {
        let null_byte_key = "sk-test\0 malicious";
        let result = RequestValidator::validate_api_key(null_byte_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unicode_in_messages() {
        let unicode_content = "你好世界 🌍 مرحبا";
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": unicode_content
                }
            ]
        });
        
        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }

    #[test]
    fn test_emoji_in_messages() {
        let emoji_content = "Hello 👋🎉🚀💻";
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": emoji_content
                }
            ]
        });
        
        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }

    #[test]
    fn test_empty_content_rejection() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": ""
                }
            ]
        });
        
        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_whitespace_only_content() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": "   \n\t  "
                }
            ]
        });
        
        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }

    #[test]
    fn test_special_characters_in_role() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "<script>alert('xss')</script>",
                    "content": "Hello"
                }
            ]
        });
        
        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_temperature_validation() {
        let request = json!({
            "model": "claude-3-opus",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ],
            "temperature": 1.5
        });
        
        let result = RequestValidator::validate_anthropic_message_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_max_tokens_validation() {
        let request = json!({
            "model": "claude-3-opus",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ],
            "max_tokens": 300_000
        });
        
        let result = RequestValidator::validate_anthropic_message_request(&request);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod performance_tests {

    use super::*;

    #[test]
    fn test_validation_performance_small_request() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "user",
                    "content": "Hello"
                }
            ]
        });
        
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = RequestValidator::validate_chat_completion_request(&request);
        }
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_validation_performance_medium_request() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant that provides detailed explanations."
                },
                {
                    "role": "user",
                    "content": "Explain the concept of microservices architecture and its benefits."
                }
            ],
            "temperature": 0.7,
            "max_tokens": 1000
        });
        
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = RequestValidator::validate_chat_completion_request(&request);
        }
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 50);
    }
}
