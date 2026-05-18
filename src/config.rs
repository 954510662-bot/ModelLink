use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::RwLock;

use crate::errors::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_config_version")]
    pub config_version: String,
    
    #[serde(default = "default_server")]
    pub server: ServerConfig,
    
    pub providers: HashMap<String, ProviderConfig>,
    
    pub mappings: HashMap<String, String>,
    
    #[serde(default = "default_logging")]
    pub logging: LoggingConfig,
    
    #[serde(default = "default_security")]
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProviderConfig {
    pub base_url: String,
    
    #[serde(default)]
    pub api_key: Option<String>,
    
    #[serde(default)]
    pub api_key_env: Option<String>,
    
    #[serde(default)]
    pub headers: HashMap<String, String>,
    
    #[serde(default)]
    pub capabilities: ModelCapabilities,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelCapabilities {
    #[serde(default = "default_true")]
    pub streaming: bool,
    
    #[serde(default = "default_true")]
    pub supports_function_calling: bool,
    
    #[serde(default = "default_true")]
    pub supports_temperature: bool,
    
    #[serde(default)]
    pub supports_top_p: bool,
    
    #[serde(default)]
    pub supports_top_k: bool,
    
    #[serde(default)]
    pub max_temperature: Option<f32>,
    
    #[serde(default)]
    pub max_top_p: Option<f32>,
    
    #[serde(default)]
    pub max_top_k: Option<u32>,
    
    #[serde(default)]
    pub parameter_aliases: HashMap<String, String>,
    
    #[serde(default)]
    pub unsupported_params: Vec<String>,
    
    #[serde(default)]
    pub max_tokens_limit: Option<u32>,
    
    #[serde(default)]
    pub supports_system_role: bool,
}

impl Default for ModelCapabilities {
    fn default() -> Self {
        Self {
            streaming: true,
            supports_function_calling: true,
            supports_temperature: true,
            supports_top_p: false,
            supports_top_k: false,
            max_temperature: None,
            max_top_p: None,
            max_top_k: None,
            parameter_aliases: HashMap::new(),
            unsupported_params: Vec::new(),
            max_tokens_limit: None,
            supports_system_role: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    
    #[serde(default)]
    pub file: Option<String>,
    
    #[serde(default = "default_log_retention_days")]
    pub retention_days: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    #[serde(default = "default_true")]
    pub audit_enabled: bool,
    
    #[serde(default)]
    pub audit_path: Option<String>,
    
    #[serde(default = "default_true")]
    pub masking_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ModelCapabilityDB {
    pub capabilities: HashMap<String, ModelCapabilities>,
}

impl ModelCapabilityDB {
    pub fn new() -> Self {
        let mut db = Self {
            capabilities: HashMap::new(),
        };
        db.register_builtin_models();
        db
    }
    
    fn register_builtin_models(&mut self) {
        self.register_openai_models();
        self.register_anthropic_models();
        self.register_deepseek_models();
        self.register_qwen_models();
    }
    
    fn register_openai_models(&mut self) {
        let gpt4 = ModelCapabilities {
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
        };
        
        let gpt35 = ModelCapabilities {
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
            max_tokens_limit: Some(16385),
            supports_system_role: true,
        };
        
        self.capabilities.insert("gpt-4".to_string(), gpt4.clone());
        self.capabilities.insert("gpt-4-turbo".to_string(), gpt4.clone());
        self.capabilities.insert("gpt-4o".to_string(), gpt4);
        self.capabilities.insert("gpt-3.5-turbo".to_string(), gpt35.clone());
        self.capabilities.insert("gpt-3.5-turbo-16k".to_string(), gpt35);
    }
    
    fn register_anthropic_models(&mut self) {
        let claude3 = ModelCapabilities {
            streaming: true,
            supports_function_calling: true,
            supports_temperature: true,
            supports_top_p: false,
            supports_top_k: true,
            max_temperature: Some(1.0),
            max_top_p: None,
            max_top_k: Some(256),
            parameter_aliases: HashMap::new(),
            unsupported_params: vec!["top_p".to_string()],
            max_tokens_limit: Some(200000),
            supports_system_role: true,
        };
        
        let claude35 = ModelCapabilities {
            streaming: true,
            supports_function_calling: true,
            supports_temperature: true,
            supports_top_p: false,
            supports_top_k: true,
            max_temperature: Some(1.0),
            max_top_p: None,
            max_top_k: Some(256),
            parameter_aliases: HashMap::new(),
            unsupported_params: vec!["top_p".to_string()],
            max_tokens_limit: Some(200000),
            supports_system_role: true,
        };
        
        self.capabilities.insert("claude-3-opus".to_string(), claude3.clone());
        self.capabilities.insert("claude-3-sonnet".to_string(), claude3.clone());
        self.capabilities.insert("claude-3-haiku".to_string(), claude3);
        self.capabilities.insert("claude-3.5-sonnet".to_string(), claude35.clone());
        self.capabilities.insert("claude-3.5-haiku".to_string(), claude35);
    }
    
    fn register_deepseek_models(&mut self) {
        let deepseek_chat = ModelCapabilities {
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
        };
        
        let deepseek_coder = ModelCapabilities {
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
            max_tokens_limit: Some(16000),
            supports_system_role: true,
        };
        
        self.capabilities.insert("deepseek-chat".to_string(), deepseek_chat);
        self.capabilities.insert("deepseek-coder".to_string(), deepseek_coder);
    }
    
    fn register_qwen_models(&mut self) {
        let mut qwen_aliases = HashMap::new();
        qwen_aliases.insert("max_tokens".to_string(), "maxTokens".to_string());
        
        let qwen = ModelCapabilities {
            streaming: true,
            supports_function_calling: true,
            supports_temperature: true,
            supports_top_p: true,
            supports_top_k: false,
            max_temperature: Some(1.0),
            max_top_p: Some(1.0),
            max_top_k: None,
            parameter_aliases: qwen_aliases,
            unsupported_params: vec![],
            max_tokens_limit: Some(32000),
            supports_system_role: true,
        };
        
        let mut qwen_coder_aliases = HashMap::new();
        qwen_coder_aliases.insert("max_tokens".to_string(), "maxTokens".to_string());
        
        let qwen_coder = ModelCapabilities {
            streaming: true,
            supports_function_calling: false,
            supports_temperature: true,
            supports_top_p: true,
            supports_top_k: false,
            max_temperature: Some(1.0),
            max_top_p: Some(1.0),
            max_top_k: None,
            parameter_aliases: qwen_coder_aliases,
            unsupported_params: vec!["tools".to_string()],
            max_tokens_limit: Some(32000),
            supports_system_role: true,
        };
        
        self.capabilities.insert("qwen-turbo".to_string(), qwen.clone());
        self.capabilities.insert("qwen-plus".to_string(), qwen.clone());
        self.capabilities.insert("qwen-max".to_string(), qwen);
        self.capabilities.insert("qwen-coder".to_string(), qwen_coder);
    }
    
    pub fn get(&self, model_name: &str) -> Option<&ModelCapabilities> {
        self.capabilities.get(model_name)
    }
    
    pub fn register(&mut self, model_name: String, capabilities: ModelCapabilities) {
        self.capabilities.insert(model_name, capabilities);
    }
}

impl Default for ModelCapabilityDB {
    fn default() -> Self {
        Self::new()
    }
}

fn default_config_version() -> String {
    "1.0".to_string()
}

fn default_server() -> ServerConfig {
    ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 9191,
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    9191
}

fn default_logging() -> LoggingConfig {
    LoggingConfig {
        level: "info".to_string(),
        file: None,
        retention_days: 30,
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_retention_days() -> u32 {
    30
}

fn default_security() -> SecurityConfig {
    SecurityConfig {
        audit_enabled: true,
        audit_path: None,
        masking_enabled: true,
    }
}

fn default_true() -> bool {
    true
}

pub struct ConfigManager {
    config: RwLock<Config>,
    config_path: PathBuf,
    capability_db: ModelCapabilityDB,
}

impl ConfigManager {
    pub async fn new(config_path: Option<PathBuf>) -> Result<Self> {
        let path = config_path.unwrap_or_else(Self::default_config_path);
        let config = Self::load_config(&path).await?;
        
        Ok(Self {
            config: RwLock::new(config),
            config_path: path,
            capability_db: ModelCapabilityDB::new(),
        })
    }
    
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("model-link")
            .join("config.yaml")
    }
    
    async fn load_config(path: &PathBuf) -> Result<Config> {
        if !path.exists() {
            return Ok(Self::create_default_config());
        }
        
        let content = tokio::fs::read_to_string(path).await?;
        let mut config: Config = serde_yaml::from_str(&content)?;
        
        Self::resolve_env_vars(&mut config);
        
        Ok(config)
    }
    
    fn resolve_env_vars(config: &mut Config) {
        for provider in config.providers.values_mut() {
            if let Some(env_var) = &provider.api_key_env {
                if let Ok(value) = std::env::var(env_var) {
                    provider.api_key = Some(value);
                }
            }
        }
    }
    
    pub fn create_default_config() -> Config {
        let mut providers = HashMap::new();
        providers.insert(
            "deepseek".to_string(),
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
            },
        );
        
        let mut mappings = HashMap::new();
        mappings.insert("claude-3-opus".to_string(), "deepseek-chat".to_string());
        mappings.insert("claude-3-sonnet".to_string(), "deepseek-chat".to_string());
        mappings.insert("gpt-4".to_string(), "deepseek-chat".to_string());
        
        Config {
            config_version: "1.0".to_string(),
            server: default_server(),
            providers,
            mappings,
            logging: default_logging(),
            security: default_security(),
        }
    }
    
    pub async fn get(&self) -> Config {
        self.config.read().await.clone()
    }
    
    pub async fn reload(&self) -> Result<()> {
        let config = Self::load_config(&self.config_path).await?;
        *self.config.write().await = config;
        Ok(())
    }
    
    pub fn get_path(&self) -> &PathBuf {
        &self.config_path
    }
    
    pub fn get_capabilities(&self, model_name: &str) -> Option<&ModelCapabilities> {
        self.capability_db.get(model_name)
    }
    
    pub fn get_all_known_models(&self) -> Vec<String> {
        self.capability_db.capabilities.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = ConfigManager::create_default_config();
        assert_eq!(config.server.port, 9191);
        assert_eq!(config.server.host, "127.0.0.1");
        assert!(config.providers.contains_key("deepseek"));
    }
    
    #[test]
    fn test_env_var_resolution() {
        std::env::set_var("TEST_API_KEY", "test-key-123");
        
        let mut config = ConfigManager::create_default_config();
        config.providers.insert(
            "test".to_string(),
            ProviderConfig {
                base_url: "https://example.com".to_string(),
                api_key: None,
                api_key_env: Some("TEST_API_KEY".to_string()),
                headers: HashMap::new(),
                capabilities: ModelCapabilities::default(),
            },
        );
        
        ConfigManager::resolve_env_vars(&mut config);
        
        assert_eq!(
            config.providers.get("test").unwrap().api_key,
            Some("test-key-123".to_string())
        );
        
        std::env::remove_var("TEST_API_KEY");
    }
    
    #[test]
    fn test_model_capability_db_builtin_models() {
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
    
    #[test]
    fn test_model_capability_db_claude() {
        let db = ModelCapabilityDB::new();
        
        let claude_caps = db.get("claude-3-opus").unwrap();
        assert!(claude_caps.supports_top_k);
        assert!(!claude_caps.supports_top_p);
        assert!(claude_caps.supports_system_role);
    }
    
    #[test]
    fn test_model_capability_db_qwen_aliases() {
        let db = ModelCapabilityDB::new();
        
        let qwen_caps = db.get("qwen-turbo").unwrap();
        assert_eq!(
            qwen_caps.parameter_aliases.get("max_tokens"),
            Some(&"maxTokens".to_string())
        );
    }
    
    #[test]
    fn test_model_capability_db_register_custom() {
        let mut db = ModelCapabilityDB::new();
        
        let custom_caps = ModelCapabilities {
            streaming: true,
            supports_function_calling: false,
            supports_temperature: true,
            supports_top_p: true,
            supports_top_k: false,
            max_temperature: Some(1.5),
            max_top_p: None,
            max_top_k: None,
            parameter_aliases: HashMap::new(),
            unsupported_params: vec!["tools".to_string()],
            max_tokens_limit: Some(8000),
            supports_system_role: true,
        };
        
        db.register("custom-model".to_string(), custom_caps);
        
        assert!(db.get("custom-model").is_some());
    }
    
    #[test]
    fn test_all_known_models() {
        let db = ModelCapabilityDB::new();
        let models = db.capabilities.keys().collect::<Vec<_>>();
        
        assert!(models.len() > 10);
        assert!(models.contains(&&"gpt-4".to_string()));
        assert!(models.contains(&&"claude-3-opus".to_string()));
        assert!(models.contains(&&"deepseek-chat".to_string()));
    }
}
