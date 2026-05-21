use model_link::*;
use serde_json::json;

#[tokio::test]
async fn test_end_to_end_chat_completion() {
    let provider = OpenAIProvider::new("test-api-key".to_string());
    
    let request = json!({
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Hello, how are you?"
            }
        ],
        "max_tokens": 100
    });

    assert_eq!(provider.name(), "openai");
    assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    assert!(provider.supports_streaming());
    assert!(provider.supports_functions());
}

#[tokio::test]
async fn test_end_to_end_anthropic() {
    let provider = AnthropicProvider::new("test-api-key".to_string());
    
    let request = json!({
        "model": "claude-3-sonnet-20240229",
        "messages": [
            {
                "role": "user",
                "content": "Hello, how are you?"
            }
        ],
        "max_tokens": 100
    });

    assert_eq!(provider.name(), "anthropic");
    assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
    assert!(provider.supports_streaming());
    assert!(!provider.supports_functions());
}

#[tokio::test]
async fn test_end_to_end_gemini() {
    let provider = GeminiProvider::new("test-api-key".to_string());
    
    let request = json!({
        "model": "gemini-pro",
        "contents": [
            {
                "parts": [
                    {
                        "text": "Hello, how are you?"
                    }
                ]
            }
        ]
    });

    assert_eq!(provider.name(), "gemini");
    assert!(provider.supports_streaming());
    assert!(!provider.supports_functions());
}

#[tokio::test]
async fn test_end_to_end_deepseek() {
    let provider = DeepSeekProvider::new("test-api-key".to_string());
    
    let request = json!({
        "model": "deepseek-chat",
        "messages": [
            {
                "role": "user",
                "content": "Hello, how are you?"
            }
        ],
        "max_tokens": 100
    });

    assert_eq!(provider.name(), "deepseek");
    assert_eq!(provider.base_url(), "https://api.deepseek.com/v1");
    assert!(provider.supports_streaming());
    assert!(provider.supports_functions());
}

#[tokio::test]
async fn test_end_to_end_cohere() {
    let provider = CohereProvider::new("test-api-key".to_string());
    
    let request = json!({
        "model": "command",
        "message": "Hello, how are you?",
        "max_tokens": 100
    });

    assert_eq!(provider.name(), "cohere");
    assert_eq!(provider.base_url(), "https://api.cohere.ai/v1");
    assert!(provider.supports_streaming());
    assert!(!provider.supports_functions());
}

#[tokio::test]
async fn test_provider_factory() {
    let openai = create_provider("openai", "test-key".to_string());
    assert_eq!(openai.name(), "openai");

    let anthropic = create_provider("anthropic", "test-key".to_string());
    assert_eq!(anthropic.name(), "anthropic");

    let gemini = create_provider("gemini", "test-key".to_string());
    assert_eq!(gemini.name(), "gemini");

    let deepseek = create_provider("deepseek", "test-key".to_string());
    assert_eq!(deepseek.name(), "deepseek");

    let cohere = create_provider("cohere", "test-key".to_string());
    assert_eq!(cohere.name(), "cohere");
}

#[test]
fn test_config_schema_validation() {
    let valid_config = json!({
        "server": {
            "host": "127.0.0.1",
            "port": 8080
        },
        "providers": {
            "openai": {
                "base_url": "https://api.openai.com/v1",
                "api_key": "test-key",
                "enabled": true,
                "timeout": 30,
                "max_retries": 3
            }
        },
        "rate_limit": {
            "enabled": true,
            "requests_per_minute": 60,
            "requests_per_hour": 1000
        }
    });

    let result = validate_config(&valid_config);
    assert!(result.is_ok(), "Valid config should pass validation");

    let invalid_config = json!({
        "server": {
            "host": "127.0.0.1",
            "port": 99999
        },
        "providers": {
            "openai": {
                "api_key": "test-key"
            }
        }
    });

    let result = validate_config(&invalid_config);
    assert!(result.is_err(), "Invalid port should fail validation");
}

#[test]
fn test_config_validator() {
    let schema = get_config_schema();
    let validator = ConfigValidator::new(schema).expect("Failed to create validator");
    
    let config = json!({
        "server": {
            "host": "0.0.0.0",
            "port": 3000
        },
        "providers": {
            "anthropic": {
                "api_key": "sk-ant-key"
            }
        }
    });

    let result = validator.validate(&config);
    assert!(result.is_ok(), "Config should be valid");
}

#[test]
fn test_websocket_state() {
    let state = WebSocketState::new("openai", "test-key".to_string());
    assert_eq!(state.provider.name(), "openai");
}

#[tokio::test]
async fn test_provider_capabilities() {
    let providers = vec![
        ("openai", true, true),
        ("anthropic", true, false),
        ("gemini", true, false),
        ("deepseek", true, true),
        ("cohere", true, false),
    ];

    for (name, supports_stream, supports_func) in providers {
        let provider = create_provider(name, "test-key".to_string());
        assert_eq!(provider.name(), name);
        assert_eq!(provider.supports_streaming(), supports_stream);
        assert_eq!(provider.supports_functions(), supports_func);
    }
}
