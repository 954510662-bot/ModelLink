use std::collections::HashMap;
use std::sync::Arc;

use model_link::{
    config::{ConfigManager, ModelCapabilityDB, ProviderConfig, ServerConfig, ModelCapabilities},
    http_client::{HttpClientPool, HttpClientConfig},
    rate_limit::{RateLimiter, RateLimitConfig},
    validation::RequestValidator,
    utils::{convert_headers, sanitize_log_input, generate_request_id},
    Result,
};

use serde_json::json;

#[cfg(test)]
mod integration_tests {

    use super::*;

    #[test]
    fn test_config_manager_default() {
        let config = ConfigManager::create_default_config();
        assert_eq!(config.server.port, 9191);
        assert_eq!(config.server.host, "127.0.0.1");
        assert!(config.providers.contains_key("deepseek"));
    }

    #[test]
    fn test_model_capability_db() {
        let db = ModelCapabilityDB::new();
        
        assert!(db.get("gpt-4").is_some());
        assert!(db.get("claude-3-opus").is_some());
        assert!(db.get("deepseek-chat").is_some());
        assert!(db.get("qwen-turbo").is_some());
        
        let gpt4_caps = db.get("gpt-4").unwrap();
        assert!(gpt4_caps.supports_function_calling);
        assert_eq!(gpt4_caps.max_temperature, Some(2.0));
        assert!(!gpt4_caps.supports_top_k);
    }

    #[tokio::test]
    async fn test_http_client_pool() {
        let config = HttpClientConfig {
            max_connections: 5,
            ..Default::default()
        };
        let pool = HttpClientPool::new(config);
        
        assert_eq!(pool.client_count(), 5);
        
        let client1 = pool.get_client();
        let client2 = pool.get_client();
        assert!(std::ptr::eq(client1, client2) == false);
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let config = RateLimitConfig {
            requests_per_second: 2,
            burst_limit: 3,
            enabled: true,
        };
        let limiter = RateLimiter::new(config);

        for _ in 0..3 {
            assert!(limiter.check_rate_limit("test-client").await.is_ok());
        }
        assert!(limiter.check_rate_limit("test-client").await.is_err());
    }

    #[test]
    fn test_request_validation_valid() {
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
    fn test_request_validation_missing_model() {
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
    }

    #[test]
    fn test_request_validation_empty_messages() {
        let request = json!({
            "model": "gpt-4",
            "messages": []
        });

        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_request_validation_temperature_range() {
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
    fn test_request_validation_invalid_role() {
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
    }

    #[test]
    fn test_anthropic_validation_valid() {
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
    fn test_anthropic_validation_system_role_invalid() {
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
    }

    #[test]
    fn test_utils_convert_headers() {
        use axum::http::HeaderMap;
        use axum::http::HeaderValue;
        
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("authorization", HeaderValue::from_static("Bearer token"));
        
        let result = convert_headers(&headers);
        
        assert!(result.contains_key("content-type"));
        assert!(result.contains_key("authorization"));
    }

    #[test]
    fn test_utils_convert_headers_excludes_host() {
        use axum::http::HeaderMap;
        use axum::http::HeaderValue;
        
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("localhost"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let result = convert_headers(&headers);
        
        assert!(!result.contains_key("host"));
        assert!(result.contains_key("content-type"));
    }

    #[test]
    fn test_utils_sanitize_log_input() {
        let input = "Hello\nWorld<script>alert('xss')</script>";
        let sanitized = sanitize_log_input(input);
        
        assert!(!sanitized.contains("<script>"));
        assert!(sanitized.contains("Hello"));
        assert!(sanitized.contains("World"));
    }

    #[test]
    fn test_utils_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36);
    }

    #[test]
    fn test_api_key_validation() {
        assert!(RequestValidator::validate_api_key("sk-1234567890abcdef").is_ok());
        assert!(RequestValidator::validate_api_key("").is_err());
        assert!(RequestValidator::validate_api_key("short").is_err());
    }

    #[test]
    fn test_provider_config_defaults() {
        let capabilities = ModelCapabilities::default();
        
        assert!(capabilities.streaming);
        assert!(capabilities.supports_function_calling);
        assert!(capabilities.supports_temperature);
        assert!(!capabilities.supports_top_p);
        assert!(!capabilities.supports_top_k);
    }

    #[test]
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        
        assert_eq!(config.port, 9191);
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        
        assert!(config.enabled);
        assert_eq!(config.requests_per_second, 10);
        assert_eq!(config.burst_limit, 50);
    }

    #[test]
    fn test_http_client_config_defaults() {
        let config = HttpClientConfig::default();
        
        assert_eq!(config.timeout_seconds, 300);
        assert_eq!(config.connect_timeout_seconds, 10);
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.idle_timeout_seconds, 30);
        assert_eq!(config.keep_alive_seconds, 30);
    }
}

#[cfg(test)]
mod edge_case_tests {

    use super::*;

    #[test]
    fn test_max_tokens_validation() {
        let request = json!({
            "model": "gpt-4",
            "messages": [{ "role": "user", "content": "Hello" }],
            "max_tokens": 2_000_000
        });
        
        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_top_p_validation() {
        let request = json!({
            "model": "gpt-4",
            "messages": [{ "role": "user", "content": "Hello" }],
            "top_p": 1.5
        });
        
        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_content_length_limit() {
        let long_content = "a".repeat(1_100_000);
        let request = json!({
            "model": "gpt-4",
            "messages": [{ 
                "role": "user", 
                "content": long_content 
            }]
        });
        
        let result = RequestValidator::validate_chat_completion_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_messages_validation() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                { "role": "system", "content": "You are helpful" },
                { "role": "user", "content": "Hello" },
                { "role": "assistant", "content": "Hi there" },
                { "role": "user", "content": "How are you?" }
            ],
            "temperature": 0.8
        });
        
        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }

    #[test]
    fn test_function_call_message_validation() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "assistant",
                    "content": null,
                    "function_call": {
                        "name": "get_weather",
                        "arguments": "{\"location\":\"Boston\"}"
                    }
                }
            ]
        });
        
        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }

    #[test]
    fn test_tool_calls_message_validation() {
        let request = json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [
                        {
                            "id": "call_123",
                            "type": "function",
                            "function": {
                                "name": "get_weather",
                                "arguments": "{\"location\":\"Boston\"}"
                            }
                        }
                    ]
                }
            ]
        });
        
        assert!(RequestValidator::validate_chat_completion_request(&request).is_ok());
    }
}
