use std::sync::Arc;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::audit::MetricsCollector;

pub type MetricsState = Arc<MetricsCollector>;

pub async fn create_metrics_router(metrics: Arc<MetricsCollector>) -> Router {
    Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(metrics)
}

async fn metrics_handler(state: axum::extract::State<MetricsState>) -> Response {
    let metrics = state.get_metrics();
    
    let output = format!(
        r#"# HELP model_link_requests_total Total number of requests
# TYPE model_link_requests_total counter
model_link_requests_total {}

# HELP model_link_errors_total Total number of errors
# TYPE model_link_errors_total counter
model_link_errors_total {}

# HELP model_link_tokens_total Total number of tokens processed
# TYPE model_link_tokens_total counter
model_link_tokens_total {}

# HELP model_link_active_streams Number of active streaming connections
# TYPE model_link_active_streams gauge
model_link_active_streams {}

# HELP model_link_request_duration_seconds_avg Average request latency in seconds
# TYPE model_link_request_duration_seconds_avg gauge
model_link_request_duration_seconds_avg {}

# HELP model_link_up Up whether model-link is running
# TYPE model_link_up gauge
model_link_up 1
"#
        ,
        metrics.total_requests,
        metrics.total_errors,
        metrics.total_tokens,
        metrics.active_streams,
        metrics.avg_latency_ms() / 1000.0,
    );
    
    (StatusCode::OK, output).into_response()
}

#[allow(dead_code)]
pub struct PrometheusFormatter;

#[allow(dead_code)]
impl PrometheusFormatter {
    pub fn format_request_duration(duration_ms: u64, buckets: &[f64]) -> String {
        let mut output = String::new();
        
        for bucket in buckets {
            let bucket_value = if *bucket <= duration_ms as f64 {
                1.0
            } else {
                0.0
            };
            output.push_str(&format!(
                "model_link_request_duration_seconds_bucket{{le=\"{}\"}} {}\n",
                bucket, bucket_value
            ));
        }
        
        output.push_str(&format!(
            "model_link_request_duration_seconds_bucket{{le=\"+Inf\"}} 1\n",
        ));
        
        output
    }
}

pub struct MetricsRecorder {
    collector: Arc<MetricsCollector>,
    request_start_times: RwLock<HashMap<String, u64>>,
}

impl MetricsRecorder {
    pub fn new(collector: Arc<MetricsCollector>) -> Self {
        Self {
            collector,
            request_start_times: RwLock::new(HashMap::new()),
        }
    }
    
    pub async fn start_request(&self, request_id: &str) {
        self.collector.record_request();
        let mut times = self.request_start_times.write().await;
        times.insert(
            request_id.to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        );
    }
    
    pub async fn end_request(&self, request_id: &str, tokens: Option<u64>, error: bool) {
        let times = self.request_start_times.read().await;
        if let Some(start_time) = times.get(request_id) {
            let end_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            
            let latency = end_time - start_time;
            self.collector.record_latency(latency);
        }
        drop(times);
        
        if let Some(t) = tokens {
            self.collector.record_tokens(t);
        }
        
        if error {
            self.collector.record_error();
        }
        
        let mut times = self.request_start_times.write().await;
        times.remove(request_id);
    }
    
    pub fn record_stream_start(&self) {
        self.collector.stream_started();
    }
    
    pub fn record_stream_end(&self) {
        self.collector.stream_ended();
    }
}

pub type RecorderState = Arc<MetricsRecorder>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prometheus_formatter_request_duration() {
        let buckets = vec![0.01, 0.05, 0.1, 0.5, 1.0, 5.0];
        
        let output = PrometheusFormatter::format_request_duration(50, &buckets);
        
        assert!(output.contains("le=\"0.05\""));
        assert!(output.contains("le=\"+Inf\""));
    }
    
    #[tokio::test]
    async fn test_metrics_recorder() {
        let collector = Arc::new(MetricsCollector::new());
        let recorder = MetricsRecorder::new(collector.clone());
        
        recorder.start_request("test-1").await;
        
        assert_eq!(collector.get_metrics().total_requests, 1);
        
        recorder.end_request("test-1", Some(100), false).await;
        
        assert_eq!(collector.get_metrics().total_tokens, 100);
    }
    
    #[tokio::test]
    async fn test_metrics_recorder_error() {
        let collector = Arc::new(MetricsCollector::new());
        let recorder = MetricsRecorder::new(collector.clone());
        
        recorder.start_request("test-error").await;
        recorder.end_request("test-error", None, true).await;
        
        assert_eq!(collector.get_metrics().total_errors, 1);
    }
}
