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

pub struct MistralProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl MistralProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "mistral".to_string(),
            base_url: "https://api.mistral.ai/v1".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for MistralProvider {
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

pub struct GroqProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl GroqProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "groq".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for GroqProvider {
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

pub struct OllamaProvider {
    name: String,
    base_url: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self {
            name: "ollama".to_string(),
            base_url: "http://localhost:11434".to_string(),
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
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
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("llama2");
        
        let ollama_request = serde_json::json!({
            "model": model,
            "messages": request.get("messages"),
            "stream": false,
        });
        
        let response = client
            .post(format!("{}/api/chat", self.base_url))
            .header("Content-Type", "application/json")
            .json(&ollama_request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("llama2");
        
        let ollama_request = serde_json::json!({
            "model": model,
            "messages": request.get("messages"),
            "stream": true,
        });
        
        let response = client
            .post(format!("{}/api/chat", self.base_url))
            .header("Content-Type", "application/json")
            .json(&ollama_request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }
}

pub struct AzureOpenAIProvider {
    name: String,
    base_url: String,
    api_key: String,
    api_version: String,
}

impl AzureOpenAIProvider {
    pub fn new(api_key: String, endpoint: String, api_version: String) -> Self {
        Self {
            name: "azure-openai".to_string(),
            base_url: endpoint,
            api_key,
            api_version,
        }
    }
}

#[async_trait]
impl Provider for AzureOpenAIProvider {
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
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gpt-4");
        
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.base_url, model, self.api_version
        );
        
        let response = client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gpt-4");
        
        let mut request = request;
        if let Some(obj) = request.as_object_mut() {
            obj.insert("stream".to_string(), Value::Bool(true));
        }
        
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.base_url, model, self.api_version
        );
        
        let response = client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }
}

pub struct PerplexityProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl PerplexityProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "perplexity".to_string(),
            base_url: "https://api.perplexity.ai".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for PerplexityProvider {
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

pub struct HuggingFaceProvider {
    name: String,
    base_url: String,
    api_key: String,
}

impl HuggingFaceProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            name: "huggingface".to_string(),
            base_url: "https://api-inference.huggingface.co".to_string(),
            api_key,
        }
    }
}

#[async_trait]
impl Provider for HuggingFaceProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_streaming(&self) -> bool {
        false
    }

    fn supports_functions(&self) -> bool {
        false
    }

    async fn chat_completions(&self, request: Value) -> Result<Value> {
        let client = reqwest::Client::new();
        let model = request.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("meta-llama/Llama-2-70b-chat-hf");
        
        let response = client
            .post(format!("{}/models/{}/v1/chat", self.base_url, model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        let result = response.json::<Value>().await?;
        Ok(result)
    }

    async fn chat_completions_stream(&self, _request: Value) -> Result<Value> {
        Err(anyhow::anyhow!("HuggingFace provider does not support streaming"))
    }
}

pub fn create_provider(provider_type: &str, api_key: String) -> Box<dyn Provider> {
    match provider_type.to_lowercase().as_str() {
        "openai" => Box::new(OpenAIProvider::new(api_key)),
        "anthropic" => Box::new(AnthropicProvider::new(api_key)),
        "gemini" => Box::new(GeminiProvider::new(api_key)),
        "deepseek" => Box::new(DeepSeekProvider::new(api_key)),
        "cohere" => Box::new(CohereProvider::new(api_key)),
        "mistral" => Box::new(MistralProvider::new(api_key)),
        "groq" => Box::new(GroqProvider::new(api_key)),
        "ollama" => Box::new(OllamaProvider::new()),
        "perplexity" => Box::new(PerplexityProvider::new(api_key)),
        "huggingface" => Box::new(HuggingFaceProvider::new(api_key)),
        _ => panic!("Unknown provider type: {}", provider_type),
    }
}

pub fn get_supported_providers() -> Vec<&'static str> {
    vec![
        "openai",
        "anthropic",
        "gemini",
        "deepseek",
        "cohere",
        "mistral",
        "groq",
        "ollama",
        "azure-openai",
        "perplexity",
        "huggingface",
    ]
}
