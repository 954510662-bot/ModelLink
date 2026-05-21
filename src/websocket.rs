use axum::{
    extract::ws::{WebSocket, Message, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use crate::provider::{Provider, create_provider};
use anyhow::Result;

pub struct WebSocketState {
    pub provider: Box<dyn Provider>,
}

impl WebSocketState {
    pub fn new(provider_type: &str, api_key: String) -> Self {
        Self {
            provider: create_provider(provider_type, api_key),
        }
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<RwLock<WebSocketState>>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<RwLock<WebSocketState>>) {
    let (mut sender, mut receiver) = socket.split();
    let state = Arc::clone(&state);

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = handle_text_message(&mut sender, &state, &text).await {
                    tracing::error!("Error handling text message: {:?}", e);
                    break;
                }
            }
            Ok(Message::Binary(data)) => {
                if let Err(e) = handle_binary_message(&mut sender, &state, &data).await {
                    tracing::error!("Error handling binary message: {:?}", e);
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket closed by client");
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {:?}", e);
                break;
            }
            _ => {
                continue;
            }
        }
    }
}

async fn handle_text_message(
    sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    state: &Arc<RwLock<WebSocketState>>,
    text: &str,
) -> Result<()> {
    let request: Value = serde_json::from_str(text)?;
    
    let streaming = request.get("stream")
        .and_then(|s| s.as_bool())
        .unwrap_or(false);

    if streaming {
        let state_read = state.read().await;
        let provider = &state_read.provider;
        
        match provider.chat_completions_stream(request).await {
            Ok(response) => {
                let response_text = serde_json::to_string(&response)?;
                sender.send(Message::Text(response_text)).await?;
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": {
                        "message": e.to_string(),
                        "type": "api_error"
                    }
                });
                let error_text = serde_json::to_string(&error_response)?;
                sender.send(Message::Text(error_text)).await?;
            }
        }
    } else {
        let state_read = state.read().await;
        let provider = &state_read.provider;
        
        match provider.chat_completions(request).await {
            Ok(response) => {
                let response_text = serde_json::to_string(&response)?;
                sender.send(Message::Text(response_text)).await?;
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": {
                        "message": e.to_string(),
                        "type": "api_error"
                    }
                });
                let error_text = serde_json::to_string(&error_response)?;
                sender.send(Message::Text(error_text)).await?;
            }
        }
    }

    Ok(())
}

async fn handle_binary_message(
    sender: &mut futures_util::stream::SplitSink<WebSocket, Message>,
    state: &Arc<RwLock<WebSocketState>>,
    data: &[u8],
) -> Result<()> {
    let request: Value = serde_json::from_slice(data)?;
    
    let state_read = state.read().await;
    let provider = &state_read.provider;
    
    match provider.chat_completions(request).await {
        Ok(response) => {
            let response_bytes = serde_json::to_vec(&response)?;
            sender.send(Message::Binary(response_bytes)).await?;
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "error": {
                    "message": e.to_string(),
                    "type": "api_error"
                }
            });
            let error_bytes = serde_json::to_vec(&error_response)?;
            sender.send(Message::Binary(error_bytes)).await?;
        }
    }

    Ok(())
}

pub fn create_websocket_router(provider_type: &str, api_key: String) -> Router {
    let state = Arc::new(RwLock::new(WebSocketState::new(provider_type, api_key)));
    
    Router::new()
        .route("/ws/chat", get(websocket_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_state_creation() {
        let state = WebSocketState::new("openai", "test-api-key".to_string());
        assert_eq!(state.provider.name(), "openai");
    }

    #[tokio::test]
    async fn test_websocket_provider_capabilities() {
        let state = WebSocketState::new("anthropic", "test-api-key".to_string());
        assert!(state.provider.supports_streaming());
        assert!(!state.provider.supports_functions());
    }
}
