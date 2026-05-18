use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MockMode {
    Off,
    Mock,
    Record,
    Replay,
}

impl Default for MockMode {
    fn default() -> Self {
        MockMode::Off
    }
}

impl std::fmt::Display for MockMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MockMode::Off => write!(f, "off"),
            MockMode::Mock => write!(f, "mock"),
            MockMode::Record => write!(f, "record"),
            MockMode::Replay => write!(f, "replay"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponse {
    pub model: String,
    pub response: serde_json::Value,
    pub latency_ms: Option<u64>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl MockResponse {
    pub fn simple_text(model: &str, content: &str) -> Self {
        Self {
            model: model.to_string(),
            response: serde_json::json!({
                "id": format!("mock-{}", uuid::Uuid::new_v4()),
                "object": "chat.completion",
                "created": chrono::Utc::now().timestamp() as u64,
                "model": model,
                "choices": [{
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": content
                    },
                    "finish_reason": "stop"
                }],
                "usage": {
                    "prompt_tokens": 10,
                    "completion_tokens": 20,
                    "total_tokens": 30
                }
            }),
            latency_ms: Some(100),
            usage: Some(Usage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            }),
        }
    }

    pub fn streaming_chunk(model: &str, content: &str) -> Vec<String> {
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut chunks = Vec::new();
        
        for (i, _) in words.iter().enumerate() {
            let chunk = format!(
                "data: {{\"id\":\"mock-{}\",\"object\":\"chat.completion.chunk\",\"created\":{},\"model\":\"{}\",\"choices\":[{{\"index\":0,\"delta\":{{\"content\":\"{} \"}},\"finish_reason\":null}}]}}\n\n",
                i,
                chrono::Utc::now().timestamp(),
                model,
                words[..=i.min(words.len()-1)].join(" ")
            );
            chunks.push(chunk);
        }
        
        chunks.push("data: [DONE]\n\n".to_string());
        chunks
    }
}

pub struct MockServer {
    mode: Arc<RwLock<MockMode>>,
    responses: Arc<RwLock<HashMap<String, MockResponse>>>,
    recordings: Arc<RwLock<Vec<(String, MockResponse)>>>,
    delay_ms: Arc<RwLock<u64>>,
}

impl MockServer {
    pub fn new() -> Self {
        Self {
            mode: Arc::new(RwLock::new(MockMode::Off)),
            responses: Arc::new(RwLock::new(HashMap::new())),
            recordings: Arc::new(RwLock::new(Vec::new())),
            delay_ms: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn set_mode(&self, mode: MockMode) {
        let mut current = self.mode.write().await;
        *current = mode.clone();
        tracing::info!("Mock 模式已切换到: {}", mode);
    }

    pub async fn get_mode(&self) -> MockMode {
        self.mode.read().await.clone()
    }

    pub async fn set_delay(&self, delay_ms: u64) {
        let mut delay = self.delay_ms.write().await;
        *delay = delay_ms;
        tracing::info!("Mock 延迟设置为: {}ms", delay_ms);
    }

    pub async fn add_response(&self, model: &str, response: MockResponse) {
        let mut responses = self.responses.write().await;
        responses.insert(model.to_string(), response);
        tracing::debug!("已添加 Mock 响应: {}", model);
    }

    pub async fn get_response(&self, model: &str) -> Option<MockResponse> {
        let responses = self.responses.read().await;
        responses.get(model).cloned()
    }

    pub async fn should_use_mock(&self) -> bool {
        let mode = self.mode.read().await;
        matches!(*mode, MockMode::Mock | MockMode::Replay)
    }

    pub async fn should_record(&self) -> bool {
        let mode = self.mode.read().await;
        matches!(*mode, MockMode::Record)
    }

    pub async fn handle_request(
        &self,
        model: &str,
        _request: &serde_json::Value,
    ) -> Option<serde_json::Value> {
        let mode = self.mode.read().await.clone();
        
        match mode {
            MockMode::Mock | MockMode::Replay => {
                if let Some(response) = self.get_response(model).await {
                    if *self.delay_ms.read().await > 0 {
                        tokio::time::sleep(
                            tokio::time::Duration::from_millis(*self.delay_ms.read().await)
                        ).await;
                    }
                    return Some(response.response);
                }
                
                tracing::warn!("Mock 模式但未找到响应，返回默认响应");
                Some(MockResponse::simple_text(model, "这是一个 Mock 响应").response)
            }
            _ => None,
        }
    }

    pub async fn record_response(&self, model: String, response: MockResponse) {
        if self.should_record().await {
            let mut recordings = self.recordings.write().await;
            recordings.push((model, response));
            tracing::debug!("已录制响应，当前录制数量: {}", recordings.len());
        }
    }

    pub async fn get_recordings(&self) -> Vec<(String, MockResponse)> {
        self.recordings.read().await.clone()
    }

    pub async fn clear_recordings(&self) {
        let mut recordings = self.recordings.write().await;
        let count = recordings.len();
        recordings.clear();
        tracing::info!("已清除 {} 条录制记录", count);
    }

    pub async fn export_recordings(&self) -> serde_json::Value {
        let recordings = self.recordings.read().await;
        serde_json::json!({
            "recordings": recordings.iter().map(|(model, resp)| {
                serde_json::json!({
                    "model": model,
                    "response": resp
                })
            }).collect::<Vec<_>>(),
            "count": recordings.len(),
            "exported_at": chrono::Utc::now()
        })
    }

    pub async fn load_recordings(&self, data: serde_json::Value) -> anyhow::Result<()> {
        let recordings = data
            .get("recordings")
            .ok_or_else(|| anyhow::anyhow!("Missing 'recordings' field"))?
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("'recordings' must be an array"))?;

        let mut responses = self.responses.write().await;
        for item in recordings {
            let model = item.get("model")
                .ok_or_else(|| anyhow::anyhow!("Missing 'model' field"))?
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("'model' must be string"))?
                .to_string();
            
            let response: MockResponse = serde_json::from_value(
                item.get("response")
                    .ok_or_else(|| anyhow::anyhow!("Missing 'response' field"))?
                    .clone()
            ).map_err(|e| anyhow::anyhow!("Invalid response format: {}", e))?;
            
            responses.insert(model, response);
        }

        tracing::info!("已加载 {} 条录制记录", recordings.len());
        Ok(())
    }
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_mode_switch() {
        let server = MockServer::new();
        
        server.set_mode(MockMode::Mock).await;
        assert_eq!(server.get_mode().await, MockMode::Mock);
        
        server.set_mode(MockMode::Record).await;
        assert_eq!(server.get_mode().await, MockMode::Record);
        
        server.set_mode(MockMode::Replay).await;
        assert_eq!(server.get_mode().await, MockMode::Replay);
    }

    #[test]
    fn test_mock_response_creation() {
        let response = MockResponse::simple_text("gpt-4", "Hello!");
        assert_eq!(response.model, "gpt-4");
        assert!(response.usage.is_some());
    }

    #[test]
    fn test_streaming_chunks() {
        let chunks = MockResponse::streaming_chunk("gpt-4", "Hello world");
        assert!(!chunks.is_empty());
        assert!(chunks.last().unwrap().contains("[DONE]"));
    }

    #[tokio::test]
    async fn test_should_use_mock() {
        let server = MockServer::new();
        
        server.set_mode(MockMode::Off).await;
        assert!(!server.should_use_mock().await);
        
        server.set_mode(MockMode::Mock).await;
        assert!(server.should_use_mock().await);
    }
}
