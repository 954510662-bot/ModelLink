use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_limit: u32,
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_limit: 50,
            enabled: true,
        }
    }
}

#[derive(Debug)]
struct ClientState {
    tokens: u32,
    last_refill: Instant,
}

pub struct RateLimiter {
    config: RateLimitConfig,
    clients: Arc<RwLock<HashMap<String, ClientState>>>,
    token_interval: Duration,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        let token_interval = Duration::from_millis(1000 / config.requests_per_second as u64);
        
        Self {
            config,
            clients: Arc::new(RwLock::new(HashMap::new())),
            token_interval,
        }
    }

    async fn get_client_state(&self, client_id: &str) -> ClientState {
        let mut clients = self.clients.write().await;
        let now = Instant::now();
        
        let state = clients.entry(client_id.to_string()).or_insert_with(|| ClientState {
            tokens: self.config.burst_limit,
            last_refill: now,
        });

        let elapsed = now.duration_since(state.last_refill);
        let tokens_to_add = (elapsed.as_millis() / self.token_interval.as_millis()) as u32;
        
        if tokens_to_add > 0 {
            state.tokens = state.tokens.saturating_add(tokens_to_add).min(self.config.burst_limit);
            state.last_refill = now;
        }

        state.clone()
    }

    async fn consume_token(&self, client_id: &str) -> bool {
        let mut clients = self.clients.write().await;
        
        if let Some(state) = clients.get_mut(client_id) {
            if state.tokens > 0 {
                state.tokens -= 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub async fn check_rate_limit(&self, client_id: &str) -> Result<(), RateLimitError> {
        if !self.config.enabled {
            return Ok(());
        }

        let state = self.get_client_state(client_id).await;
        
        if state.tokens == 0 {
            return Err(RateLimitError::Limited);
        }

        self.consume_token(client_id).await;
        Ok(())
    }
}

#[derive(Debug)]
pub enum RateLimitError {
    Limited,
}

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": {
                "message": "Rate limit exceeded. Please try again later.",
                "type": "rate_limit_error",
            }
        });
        
        (StatusCode::TOO_MANY_REQUESTS, axum::Json(body)).into_response()
    }
}

pub async fn rate_limit_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, RateLimitError> {
    let state = request.extensions().get::<Arc<RateLimiter>>()
        .ok_or(RateLimitError::Limited)?;
    
    let client_id = request.headers()
        .get("x-client-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default");
    
    state.check_rate_limit(client_id).await?;
    
    Ok(next.run(request).await)
}

pub type RateLimitState = Arc<RateLimiter>;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_rate_limit_basic() {
        let config = RateLimitConfig {
            requests_per_second: 2,
            burst_limit: 3,
            enabled: true,
        };
        let limiter = RateLimiter::new(config);

        assert!(limiter.check_rate_limit("test-client").await.is_ok());
        assert!(limiter.check_rate_limit("test-client").await.is_ok());
        assert!(limiter.check_rate_limit("test-client").await.is_ok());
        assert!(limiter.check_rate_limit("test-client").await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limit_refill() {
        let config = RateLimitConfig {
            requests_per_second: 10,
            burst_limit: 5,
            enabled: true,
        };
        let limiter = RateLimiter::new(config);

        for _ in 0..5 {
            assert!(limiter.check_rate_limit("test-refill").await.is_ok());
        }
        assert!(limiter.check_rate_limit("test-refill").await.is_err());

        sleep(Duration::from_millis(600)).await;
        
        assert!(limiter.check_rate_limit("test-refill").await.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limit_disabled() {
        let config = RateLimitConfig {
            requests_per_second: 1,
            burst_limit: 1,
            enabled: false,
        };
        let limiter = RateLimiter::new(config);

        for _ in 0..100 {
            assert!(limiter.check_rate_limit("test-disabled").await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limit_different_clients() {
        let config = RateLimitConfig {
            requests_per_second: 1,
            burst_limit: 2,
            enabled: true,
        };
        let limiter = RateLimiter::new(config);

        assert!(limiter.check_rate_limit("client-a").await.is_ok());
        assert!(limiter.check_rate_limit("client-a").await.is_ok());
        assert!(limiter.check_rate_limit("client-a").await.is_err());

        assert!(limiter.check_rate_limit("client-b").await.is_ok());
        assert!(limiter.check_rate_limit("client-b").await.is_ok());
        assert!(limiter.check_rate_limit("client-b").await.is_err());
    }
}