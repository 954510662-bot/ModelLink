use std::collections::HashMap;

use crate::config::{Config, ModelCapabilities, ProviderConfig, ServerConfig};

pub struct ConfigWizard;

impl ConfigWizard {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_config(&self, answers: &WizardAnswers) -> Config {
        let mut providers = HashMap::new();
        let mut mappings = HashMap::new();
        
        for (name, provider) in &answers.providers {
            providers.insert(name.clone(), provider.clone());
        }
        
        for (source, target) in &answers.mappings {
            mappings.insert(source.clone(), target.clone());
        }
        
        Config {
            config_version: "1.0".to_string(),
            server: ServerConfig {
                host: answers.host.clone(),
                port: answers.port,
            },
            providers,
            mappings,
            logging: crate::config::LoggingConfig {
                level: answers.log_level.clone(),
                file: answers.log_file.clone(),
                retention_days: 30,
            },
            security: crate::config::SecurityConfig {
                audit_enabled: answers.audit_enabled,
                audit_path: answers.audit_path.clone(),
                masking_enabled: answers.masking_enabled,
            },
        }
    }
    
    pub fn generate_yaml(&self, config: &Config) -> String {
        serde_yaml::to_string(config).unwrap_or_default()
    }
    
    pub fn default_deepseek_config() -> ProviderConfig {
        ProviderConfig {
            base_url: "https://api.deepseek.com/v1".to_string(),
            api_key: None,
            api_key_env: Some("DEEPSEEK_API_KEY".to_string()),
            headers: HashMap::new(),
            capabilities: ModelCapabilities {
                streaming: true,
                supports_function_calling: true,
                supports_temperature: true,
                supports_top_p: true,
                supports_top_k: false,
                max_temperature: Some(1.0),
                max_top_p: Some(1.0),
                max_top_k: None,
                parameter_aliases: HashMap::new(),
                unsupported_params: vec![],
                max_tokens_limit: Some(64000),
                supports_system_role: true,
            },
        }
    }
    
    pub fn default_qwen_config() -> ProviderConfig {
        let mut aliases = HashMap::new();
        aliases.insert("max_tokens".to_string(), "maxTokens".to_string());
        
        ProviderConfig {
            base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
            api_key: None,
            api_key_env: Some("QWEN_API_KEY".to_string()),
            headers: HashMap::new(),
            capabilities: ModelCapabilities {
                streaming: true,
                supports_function_calling: true,
                supports_temperature: true,
                supports_top_p: true,
                supports_top_k: false,
                max_temperature: Some(1.0),
                max_top_p: Some(1.0),
                max_top_k: None,
                parameter_aliases: aliases,
                unsupported_params: vec![],
                max_tokens_limit: Some(32000),
                supports_system_role: true,
            },
        }
    }
    
    pub fn default_openai_config() -> ProviderConfig {
        ProviderConfig {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: None,
            api_key_env: Some("OPENAI_API_KEY".to_string()),
            headers: HashMap::new(),
            capabilities: ModelCapabilities {
                streaming: true,
                supports_function_calling: true,
                supports_temperature: true,
                supports_top_p: true,
                supports_top_k: false,
                max_temperature: Some(2.0),
                max_top_p: Some(1.0),
                max_top_k: None,
                parameter_aliases: HashMap::new(),
                unsupported_params: vec!["top_k".to_string()],
                max_tokens_limit: Some(128000),
                supports_system_role: true,
            },
        }
    }
}

impl Default for ConfigWizard {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct WizardAnswers {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub log_file: Option<String>,
    pub audit_enabled: bool,
    pub audit_path: Option<String>,
    pub masking_enabled: bool,
    pub providers: HashMap<String, ProviderConfig>,
    pub mappings: HashMap<String, String>,
}

impl Default for WizardAnswers {
    fn default() -> Self {
        let mut providers = HashMap::new();
        providers.insert("deepseek".to_string(), ConfigWizard::default_deepseek_config());
        
        let mut mappings = HashMap::new();
        mappings.insert("claude-3-opus".to_string(), "deepseek-chat".to_string());
        mappings.insert("gpt-4".to_string(), "deepseek-chat".to_string());
        
        Self {
            host: "127.0.0.1".to_string(),
            port: 9191,
            log_level: "info".to_string(),
            log_file: None,
            audit_enabled: true,
            audit_path: None,
            masking_enabled: true,
            providers,
            mappings,
        }
    }
}

pub struct QuickSetup {
    pub provider_type: ProviderType,
    pub api_key_env: String,
    pub default_mapping: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderType {
    DeepSeek,
    Qwen,
    OpenAI,
    Custom,
}

impl ProviderType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "deepseek" | "ds" => Some(ProviderType::DeepSeek),
            "qwen" | "aliyun" | "ali" => Some(ProviderType::Qwen),
            "openai" | "open" | "ai" => Some(ProviderType::OpenAI),
            "custom" => Some(ProviderType::Custom),
            _ => None,
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            ProviderType::DeepSeek => "DeepSeek",
            ProviderType::Qwen => "阿里云通义千问",
            ProviderType::OpenAI => "OpenAI",
            ProviderType::Custom => "自定义",
        }
    }
    
    pub fn get_config(&self) -> ProviderConfig {
        match self {
            ProviderType::DeepSeek => ConfigWizard::default_deepseek_config(),
            ProviderType::Qwen => ConfigWizard::default_qwen_config(),
            ProviderType::OpenAI => ConfigWizard::default_openai_config(),
            ProviderType::Custom => ProviderConfig {
                base_url: "https://api.example.com/v1".to_string(),
                api_key: None,
                api_key_env: Some("CUSTOM_API_KEY".to_string()),
                headers: HashMap::new(),
                capabilities: ModelCapabilities::default(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_wizard_generate_config() {
        let wizard = ConfigWizard::new();
        let answers = WizardAnswers::default();
        
        let config = wizard.generate_config(&answers);
        
        assert_eq!(config.server.port, 9191);
        assert_eq!(config.server.host, "127.0.0.1");
        assert!(config.providers.contains_key("deepseek"));
        assert!(config.mappings.contains_key("claude-3-opus"));
    }
    
    #[test]
    fn test_provider_type_from_str() {
        assert_eq!(ProviderType::from_str("deepseek"), Some(ProviderType::DeepSeek));
        assert_eq!(ProviderType::from_str("ds"), Some(ProviderType::DeepSeek));
        assert_eq!(ProviderType::from_str("qwen"), Some(ProviderType::Qwen));
        assert_eq!(ProviderType::from_str("openai"), Some(ProviderType::OpenAI));
        assert_eq!(ProviderType::from_str("unknown"), None);
    }
    
    #[test]
    fn test_provider_type_display_name() {
        assert_eq!(ProviderType::DeepSeek.display_name(), "DeepSeek");
        assert_eq!(ProviderType::Qwen.display_name(), "阿里云通义千问");
        assert_eq!(ProviderType::OpenAI.display_name(), "OpenAI");
    }
    
    #[test]
    fn test_generate_yaml() {
        let wizard = ConfigWizard::new();
        let config = Config {
            config_version: "1.0".to_string(),
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 9191,
            },
            providers: HashMap::new(),
            mappings: HashMap::new(),
            logging: crate::config::LoggingConfig {
                level: "info".to_string(),
                file: None,
                retention_days: 30,
            },
            security: crate::config::SecurityConfig {
                audit_enabled: true,
                audit_path: None,
                masking_enabled: true,
            },
        };
        
        let yaml = wizard.generate_yaml(&config);
        
        assert!(yaml.contains("config_version"));
        assert!(yaml.contains("host"));
        assert!(yaml.contains("9191"));
    }
}
