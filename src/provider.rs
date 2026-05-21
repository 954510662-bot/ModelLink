use async_trait::async_trait;
use serde_json::Value;
use anyhow::Result;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
    fn supports_streaming(&self) -> bool;
    fn supports_functions(&self) -> bool;
    
    async fn chat_completions(&self, request: Value) -> Result<Value>;
    async fn chat_completions_stream(&self, request: Value) -> Result<Value>;
}

pub struct OpenAIProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "openai".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_functions(&self) -> bool {
        true
    }

    async fn chat_completions(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let mut request = request;
        
        if let Some(obj) = request.as_object_mut() {
            obj.insert("stream".to_string(), Value::Bool(true));
        }
        
        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }
}

pub struct AnthropicProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "anthropic".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_functions(&self) -> bool {
        false
    }

    async fn chat_completions(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        self.chat_completions(request).await
    }
}

pub struct GeminiProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "gemini".to_string(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_functions(&self) -> bool {
        false
    }

    async fn chat_completions(&self, request: Value) -> Result<Value> {
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gemini-pro");
        
        let client = reqwest::Client::new();
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, model, self.api_key
        );
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gemini-pro");
        
        let client = reqwest::Client::new();
        let url = format!(
            "{}/models/{}:streamGenerateContent?key={}",
            self.base_url, model, self.api_key
        );
        
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }
}

pub struct DeepSeekProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl DeepSeekProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "deepseek".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for DeepSeekProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_functions(&self) -> bool {
        true
    }

    async fn chat_completions(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let mut request = request;
        
        if let Some(obj) = request.as_object_mut() {
            obj.insert("stream".to_string(), Value::Bool(true));
        }
        
        let response = client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }
}

pub struct CohereProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl CohereProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "cohere".to_string(),
            base_url: "https://api.cohere.ai/v1".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for CohereProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_functions(&self) -> bool {
        false
    }

    async fn chat_completions(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/chat", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("accept", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let mut request = request;
        
        if let Some(obj) = request.as_object_mut() {
            obj.insert("stream".to_string(), Value::Bool(true));
        }
        
        let response = client
            .post(format!("{}/chat", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("accept", "application/x-ndjson")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }
}

pub fn create_provider(provider_type: &str, api_key: String) -> Box<dyn Provider> {
    match provider_type.to_lowercase().as_str() {
        "openai" => Box::new(OpenAIProvider::new(api_key)),
        "anthropic" => Box::new(AnthropicProvider::new(api_key)),
        "gemini" => Box::new(GeminiProvider::new(api_key)),
        "deepseek" => Box::new(DeepSeekProvider::new(api_key)),
        "cohere" => Box::new(CohereProvider::new(api_key)),
        _ => panic!("Unknown provider type: {}", provider_type),
    }
}
