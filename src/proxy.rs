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
    utils::convert_headers,
    validation::RequestValidator,
    rate_limit::{RateLimiter, RateLimitConfig},
    http_client::HttpClientPool,
};

pub struct AppState {
    pub config: Arc<ConfigManager>,
    pub metrics: Arc<MetricsCollector>,
    pub recorder: Arc<MetricsRecorder>,
    pub http_pool: Arc<HttpClientPool>,
    pub rate_limiter: Arc<RateLimiter>,
}

pub async fn create_router(
    config_manager: Arc<ConfigManager>,
    http_pool: Arc<HttpClientPool>,
    rate_limiter: Arc<RateLimiter>,
) -> Router {
    let metrics_collector = Arc::new(MetricsCollector::new());
    let metrics_recorder = Arc::new(MetricsRecorder::new(metrics_collector.clone()));
    
    let app_state = Arc::new(AppState {
        config: config_manager,
        metrics: metrics_collector.clone(),
        recorder: metrics_recorder.clone(),
        http_pool,
        rate_limiter,
    });

    let metrics_router = create_metrics_router(metrics_collector).await;
    
    Router::new()
        .route("/v1/chat/completions", post(chat_completions_handler))
        .route("/v1/messages", post(anthropic_messages_handler))
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .with_state(app_state)
        .layer(axum::middleware::from_fn_with_state(
            Arc::new(RateLimitConfig::default()),
            rate_limit::rate_limit_middleware,
        ))
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
    
    if let Err(e) = RequestValidator::validate_chat_completion_request(&request_body) {
        state.recorder.end_request(&request_id, None, true).await;
        return Err(e);
    }
    
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
        let result = forward_non_streaming_request(&state.http_pool, transformed_body, upstream_url, request_headers).await;
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
    
    let request_body: serde_json::Value = match serde_json::from_str(&body) {
        Ok(body) => body,
        Err(e) => {
            state.recorder.end_request(&request_id, None, true).await;
            return Err(ModelLinkError::ValidationError(format!("Invalid request body: {}", e)));
        }
    };
    
    if let Err(e) = RequestValidator::validate_anthropic_message_request(&request_body) {
        state.recorder.end_request(&request_id, None, true).await;
        return Err(e);
    }
    
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
    
    let client = state.http_pool.get_client();
    
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
    http_pool: &Arc<HttpClientPool>,
    request_body: serde_json::Value,
    upstream_url: String,
    headers: HeaderMap,
) -> Result<Response, ModelLinkError> {
    let client = http_pool.get_client();
    
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
