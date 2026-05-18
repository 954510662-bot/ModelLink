use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tracing;

use crate::config::SecurityConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub client_ip: String,
    pub target_model: String,
    pub request_method: String,
    pub request_path: String,
    pub response_status: u16,
    pub latency_ms: u64,
    pub tokens_used: Option<TokenUsage>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

pub struct AuditLogger {
    enabled: bool,
    file_path: Option<PathBuf>,
    file: Arc<Mutex<Option<File>>>,
    masking_enabled: bool,
}

impl AuditLogger {
    pub fn new(config: &SecurityConfig) -> Self {
        let enabled = config.audit_enabled;
        let masking_enabled = config.masking_enabled;
        let file_path = config.audit_path.as_ref().map(PathBuf::from);
        
        Self {
            enabled,
            file_path,
            file: Arc::new(Mutex::new(None)),
            masking_enabled,
        }
    }
    
    pub async fn init(&self) -> anyhow::Result<()> {
        if !self.enabled {
            tracing::info!("审计日志已禁用");
            return Ok(());
        }
        
        if let Some(path) = &self.file_path {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await?;
            
            let mut guard = self.file.lock().await;
            *guard = Some(file);
            
            tracing::info!("审计日志已启用: {}", path.display());
        }
        
        Ok(())
    }
    
    pub async fn log(&self, entry: AuditEntry) -> anyhow::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let mut entry_to_log = entry;
        
        if self.masking_enabled {
            entry_to_log.mask_sensitive_data();
        }
        
        let json = serde_json::to_string(&entry_to_log)?;
        let line = format!("{}\n", json);
        
        let mut guard = self.file.lock().await;
        if let Some(file) = guard.as_mut() {
            file.write_all(line.as_bytes()).await?;
        }
        
        tracing::debug!("审计日志已记录: {} -> {}", entry_to_log.client_ip, entry_to_log.target_model);
        
        Ok(())
    }
    
    pub async fn log_request(
        &self,
        client_ip: &str,
        target_model: &str,
        request_method: &str,
        request_path: &str,
        response_status: u16,
        latency_ms: u64,
        tokens_used: Option<TokenUsage>,
        error_message: Option<String>,
    ) -> anyhow::Result<()> {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            client_ip: client_ip.to_string(),
            target_model: target_model.to_string(),
            request_method: request_method.to_string(),
            request_path: request_path.to_string(),
            response_status,
            latency_ms,
            tokens_used,
            error_message,
        };
        
        self.log(entry).await
    }
    
    pub async fn get_recent_entries(&self, limit: usize) -> anyhow::Result<Vec<AuditEntry>> {
        let mut entries = Vec::new();
        
        if let Some(path) = &self.file_path {
            if path.exists() {
                let file = File::open(path).await?;
                let reader = BufReader::new(file);
                let mut lines = reader.lines();
                
                let mut count = 0;
                while let Ok(Some(line)) = lines.next_line().await {
                    if count >= limit {
                        break;
                    }
                    if let Ok(entry) = serde_json::from_str::<AuditEntry>(&line) {
                        entries.push(entry);
                        count += 1;
                    }
                }
            }
        }
        
        Ok(entries)
    }
}

impl AuditEntry {
    pub fn mask_sensitive_data(&mut self) {
        self.client_ip = self.mask_ip(&self.client_ip);
    }
    
    fn mask_ip(&self, ip: &str) -> String {
        if ip.contains('.') {
            let parts: Vec<&str> = ip.split('.').collect();
            if parts.len() == 4 {
                return format!("{}.{}.{}.***", parts[0], parts[1], parts[2]);
            }
        }
        "***.***.***".to_string()
    }
}

#[allow(dead_code)]
pub struct RequestIdGenerator {
    counter: std::sync::atomic::AtomicU64,
}

#[allow(dead_code)]
impl RequestIdGenerator {
    pub fn new() -> Self {
        Self {
            counter: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    pub fn generate(&self) -> String {
        let count = self.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        format!("req_{}_{}", timestamp, count)
    }
}

impl Default for RequestIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MetricsCollector {
    pub total_requests: std::sync::atomic::AtomicU64,
    pub total_errors: std::sync::atomic::AtomicU64,
    pub total_tokens: std::sync::atomic::AtomicU64,
    pub active_streams: std::sync::atomic::AtomicU64,
    pub total_latency_ms: std::sync::atomic::AtomicU64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            total_requests: std::sync::atomic::AtomicU64::new(0),
            total_errors: std::sync::atomic::AtomicU64::new(0),
            total_tokens: std::sync::atomic::AtomicU64::new(0),
            active_streams: std::sync::atomic::AtomicU64::new(0),
            total_latency_ms: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    pub fn record_request(&self) {
        self.total_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn record_error(&self) {
        self.total_errors.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn record_tokens(&self, tokens: u64) {
        self.total_tokens.fetch_add(tokens, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn record_latency(&self, latency_ms: u64) {
        self.total_latency_ms.fetch_add(latency_ms, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn stream_started(&self) {
        self.active_streams.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn stream_ended(&self) {
        self.active_streams.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    pub fn get_metrics(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_requests: self.total_requests.load(std::sync::atomic::Ordering::Relaxed),
            total_errors: self.total_errors.load(std::sync::atomic::Ordering::Relaxed),
            total_tokens: self.total_tokens.load(std::sync::atomic::Ordering::Relaxed),
            active_streams: self.active_streams.load(std::sync::atomic::Ordering::Relaxed),
            total_latency_ms: self.total_latency_ms.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub total_errors: u64,
    pub total_tokens: u64,
    pub active_streams: u64,
    pub total_latency_ms: u64,
}

impl MetricsSnapshot {
    pub fn avg_latency_ms(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_latency_ms as f64 / self.total_requests as f64
    }
    
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_errors as f64 / self.total_requests as f64 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_entry_mask_ip() {
        let mut entry = AuditEntry {
            timestamp: Utc::now(),
            client_ip: "192.168.1.100".to_string(),
            target_model: "gpt-4".to_string(),
            request_method: "POST".to_string(),
            request_path: "/v1/chat/completions".to_string(),
            response_status: 200,
            latency_ms: 100,
            tokens_used: None,
            error_message: None,
        };
        
        entry.mask_sensitive_data();
        
        assert_eq!(entry.client_ip, "192.168.1.***");
    }
    
    #[test]
    fn test_request_id_generator() {
        let generator = RequestIdGenerator::new();
        
        let id1 = generator.generate();
        let id2 = generator.generate();
        
        assert_ne!(id1, id2);
        assert!(id1.starts_with("req_"));
        assert!(id2.starts_with("req_"));
    }
    
    #[test]
    fn test_metrics_collector() {
        let metrics = MetricsCollector::new();
        
        metrics.record_request();
        metrics.record_request();
        metrics.record_error();
        metrics.record_tokens(1000);
        metrics.record_latency(50);
        metrics.record_latency(100);
        
        let snapshot = metrics.get_metrics();
        
        assert_eq!(snapshot.total_requests, 2);
        assert_eq!(snapshot.total_errors, 1);
        assert_eq!(snapshot.total_tokens, 1000);
        assert_eq!(snapshot.total_latency_ms, 150);
        assert_eq!(snapshot.avg_latency_ms(), 75.0);
        assert_eq!(snapshot.error_rate(), 50.0);
    }
    
    #[test]
    fn test_active_streams() {
        let metrics = MetricsCollector::new();
        
        assert_eq!(metrics.get_metrics().active_streams, 0);
        
        metrics.stream_started();
        metrics.stream_started();
        assert_eq!(metrics.get_metrics().active_streams, 2);
        
        metrics.stream_ended();
        assert_eq!(metrics.get_metrics().active_streams, 1);
    }
}
