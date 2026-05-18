use axum::{
    body::Body,
    extract::State,
    http::HeaderMap,
    response::Response,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid;

use crate::{
    config::ConfigManager,
    errors::ModelLinkError,
    health::{health_handler, ready_handler},
    metrics::{create_metrics_router, MetricsRecorder},
    stream::{forward_streaming_request, transform_openai_request},
    audit::MetricsCollector,
};

pub struct AppState {
    pub config: Arc<ConfigManager>,
    #[allow(dead_code)]
    pub metrics: Arc<MetricsCollector>,
    pub recorder: Arc<MetricsRecorder>,
}

pub async fn create_router(config_manager: Arc<ConfigManager>) -> Router {
    let metrics_collector = Arc::new(MetricsCollector::new());
    let metrics_recorder = Arc::new(MetricsRecorder::new(metrics_collector.clone()));
    
    let app_state = Arc::new(AppState {
        config: config_manager,
        metrics: metrics_collector.clone(),
        recorder: metrics_recorder.clone(),
    });

    let metrics_router = create_metrics_router(metrics_collector).await;
    
    Router::new()
        .route("/v1/chat/completions", post(chat_completions_handler))
        .route("/v1/messages", post(anthropic_messages_handler))
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .with_state(app_state)
        .merge(metrics_router)
}

async fn chat_completions_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, ModelLinkError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    state.recorder.start_request(&request_id).await;
    
    let config = state.config.get().await;
    
    let request_body: serde_json::Value = match serde_json::from_str(&body) {
        Ok(body) => body,
        Err(e) => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::ValidationError(format!("Invalid request body: {}", e)));
        }
    };
    
    let model_name = match request_body.get("model").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::ValidationError("Missing 'model' field".to_string()));
        }
    };
    
    let mapped_model = config.mappings.get(model_name).unwrap_or(&model_name.to_string()).clone();
    
    let provider = match config.providers.iter().next() {
        Some(p) => p,
        None => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::NotFoundError("No suitable provider found".to_string()));
        }
    };
    
    let upstream_url = format!("{}/chat/completions", provider.1.base_url);
    
    let mut capabilities = std::collections::HashMap::new();
    capabilities.insert("supports_temperature".to_string(), provider.1.capabilities.supports_temperature);
    capabilities.insert("supports_top_p".to_string(), provider.1.capabilities.supports_top_p);
    capabilities.insert("supports_top_k".to_string(), provider.1.capabilities.supports_top_k);
    
    let mut transformed_body = request_body.clone();
    if let Some(obj) = transformed_body.as_object_mut() {
        obj.insert("model".to_string(), json!(mapped_model));
    }
    transformed_body = transform_openai_request(transformed_body, &capabilities);
    
    let mut request_headers = headers.clone();
    if let Some(api_key) = &provider.1.api_key {
        if let Ok(value) = format!("Bearer {}", api_key).parse() {
            request_headers.insert("authorization", value);
        }
    }
    
    let is_streaming = request_body.get("stream").and_then(|v| v.as_bool()).unwrap_or(false);
    
    if is_streaming {
        state.recorder.record_stream_start();
        let result = forward_streaming_request(&config, model_name, transformed_body, upstream_url, request_headers).await;
        state.recorder.record_stream_end();
        if result.is_err() {
            state.recorder.end_request(&request_id, None, true).await;
        } else {
            state.recorder.end_request(&request_id, None, false).await;
        }
        result
    } else {
        let result = forward_non_streaming_request(&config, model_name, transformed_body, upstream_url, request_headers).await;
        if result.is_err() {
            state.recorder.end_request(&request_id, None, true).await;
        } else {
            state.recorder.end_request(&request_id, None, false).await;
        }
        result
    }
}

async fn anthropic_messages_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, ModelLinkError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    state.recorder.start_request(&request_id).await;
    
    let config = state.config.get().await;
    
    let _request_body: serde_json::Value = match serde_json::from_str(&body) {
        Ok(body) => body,
        Err(e) => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::ValidationError(format!("Invalid request body: {}", e)));
        }
    };
    
    let provider = match config.providers.iter().next() {
        Some(p) => p,
        None => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::NotFoundError("No provider configured".to_string()));
        }
    };
    
    let upstream_url = format!("{}/messages", provider.1.base_url);
    
    let mut request_headers = headers.clone();
    if let Some(api_key) = &provider.1.api_key {
        if let Ok(value) = api_key.parse() {
            request_headers.insert("x-api-key", value);
        }
    }
    
    let client = reqwest::Client::new();
    
    let response = match client
        .post(&upstream_url)
        .headers(convert_headers(&request_headers))
        .body(body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::NetworkError(e.to_string()));
        }
    };
    
    if !response.status().is_success() {
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => {
                state.recorder.end_request(&request_id, None, true).await;
                return Err(ModelLinkError::NetworkError(e.to_string()));
            }
        };
        state.recorder.end_request(&request_id, None, true).await;
        return Err(ModelLinkError::ProtocolError(format!(
            "Upstream returned error: {}",
            body
        )));
    }
    
    let body = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::NetworkError(e.to_string()));
        }
    };
    
    state.recorder.end_request(&request_id, None, false).await;
    Ok(Response::new(Body::from(body)))
}

async fn forward_non_streaming_request(
    _config: &crate::config::Config,
    _model_name: &str,
    request_body: serde_json::Value,
    upstream_url: String,
    headers: HeaderMap,
) -> Result<Response, ModelLinkError> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(&upstream_url)
        .headers(convert_headers(&headers))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| ModelLinkError::NetworkError(e.to_string()))?;
    
    if !response.status().is_success() {
        let body = response.text().await.map_err(|e| ModelLinkError::NetworkError(e.to_string()))?;
        return Err(ModelLinkError::ProtocolError(format!(
            "Upstream returned error: {}",
            body
        )));
    }
    
    let body = response.bytes().await.map_err(|e| ModelLinkError::NetworkError(e.to_string()))?;
    Ok(Response::new(Body::from(body)))
}

fn convert_headers(headers: &HeaderMap) -> reqwest::header::HeaderMap {
    let mut result = reqwest::header::HeaderMap::new();
    for (key, value) in headers.iter() {
        let key_lower = key.as_str().to_lowercase();
        if key_lower != "host" && key_lower != "content-length" {
            if let Ok(name) = reqwest::header::HeaderName::try_from(key.as_str()) {
                if let Ok(val) = value.to_str() {
                    if let Ok(parsed) = val.parse() {
                        result.insert(name, parsed);
                    }
                }
            }
        }
    }
    result
}
