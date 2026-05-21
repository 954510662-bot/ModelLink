use std::sync::Arc;
use std::time::Duration;

use reqwest::{Client, ClientBuilder};

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout_seconds: u64,
    pub connect_timeout_seconds: u64,
    pub max_connections: usize,
    pub idle_timeout_seconds: u64,
    pub keep_alive_seconds: u64,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 300,
            connect_timeout_seconds: 10,
            max_connections: 100,
            idle_timeout_seconds: 30,
            keep_alive_seconds: 30,
        }
    }
}

pub struct HttpClientPool {
    clients: Arc<Vec<Client>>,
    current_index: std::sync::atomic::AtomicUsize,
}

impl HttpClientPool {
    pub fn new(config: HttpClientConfig) -> Self {
        let mut clients = Vec::with_capacity(config.max_connections.min(10));
        
        for _ in 0..config.max_connections.min(10) {
            let client = Self::create_client(&config);
            clients.push(client);
        }
        
        Self {
            clients: Arc::new(clients),
            current_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn create_client(config: &HttpClientConfig) -> Client {
        ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(config.connect_timeout_seconds))
            .idle_timeout(Duration::from_secs(config.idle_timeout_seconds))
            .keep_alive(Duration::from_secs(config.keep_alive_seconds))
            .http2_prior_knowledge()
            .build()
            .expect("Failed to create HTTP client")
    }

    pub fn get_client(&self) -> &Client {
        let index = self.current_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        &self.clients[index % self.clients.len()]
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
}

pub type HttpClientPoolState = Arc<HttpClientPool>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_pool_creation() {
        let config = HttpClientConfig::default();
        let pool = HttpClientPool::new(config);
        
        assert!(pool.client_count() > 0);
        assert!(pool.client_count() <= 10);
    }

    #[test]
    fn test_http_client_pool_round_robin() {
        let config = HttpClientConfig {
            max_connections: 3,
            ..Default::default()
        };
        let pool = HttpClientPool::new(config);
        
        let client1 = pool.get_client();
        let client2 = pool.get_client();
        let client3 = pool.get_client();
        let client4 = pool.get_client();
        
        assert!(std::ptr::eq(client1, client4));
    }

    #[test]
    fn test_http_client_config_defaults() {
        let config = HttpClientConfig::default();
        
        assert_eq!(config.timeout_seconds, 300);
        assert_eq!(config.connect_timeout_seconds, 10);
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.idle_timeout_seconds, 30);
        assert_eq!(config.keep_alive_seconds, 30);
    }

    #[test]
    fn test_http_client_custom_config() {
        let config = HttpClientConfig {
            timeout_seconds: 60,
            connect_timeout_seconds: 5,
            max_connections: 5,
            idle_timeout_seconds: 10,
            keep_alive_seconds: 15,
        };
        
        let pool = HttpClientPool::new(config);
        
        assert_eq!(pool.client_count(), 5);
    }
}