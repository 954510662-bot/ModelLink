use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use serde::{Deserialize, Serialize};
use tracing;

use crate::config::ProviderConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub provider_name: String,
    pub is_healthy: bool,
    pub latency_ms: Option<u64>,
    pub error_message: Option<String>,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    pub enabled: bool,
    pub health_check_interval_secs: u64,
    pub health_check_timeout_secs: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            health_check_interval_secs: 30,
            health_check_timeout_secs: 5,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

pub struct FailoverManager {
    config: FailoverConfig,
    providers: Arc<RwLock<Vec<(String, ProviderConfig)>>>,
    health_status: Arc<RwLock<Vec<HealthCheckResult>>>,
    active_provider: Arc<RwLock<Option<String>>>,
}

impl FailoverManager {
    pub fn new(config: FailoverConfig, providers: Vec<(String, ProviderConfig)>) -> Self {
        let active = providers.first().map(|(name, _)| name.clone());
        Self {
            config,
            providers: Arc::new(RwLock::new(providers)),
            health_status: Arc::new(RwLock::new(Vec::new())),
            active_provider: Arc::new(RwLock::new(active)),
        }
    }

    pub async fn start_health_checks(&self) {
        if !self.config.enabled {
            tracing::info!("Failover is disabled");
            return;
        }

        let providers = self.providers.read().await;
        let interval_duration = Duration::from_secs(self.config.health_check_interval_secs);
        drop(providers);

        let providers_arc = self.providers.clone();
        let health_status_arc = self.health_status.clone();
        let active_arc = self.active_provider.clone();
        let timeout = Duration::from_secs(self.config.health_check_timeout_secs);
        let max_retries = self.config.max_retries;
        let retry_delay_ms = self.config.retry_delay_ms;

        tokio::spawn(async move {
            let mut timer = interval(interval_duration);
            timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            
            loop {
                timer.tick().await;
                
                let providers_snapshot = providers_arc.read().await;
                let mut health_checks = Vec::new();
                let mut all_unhealthy = true;

                for (name, provider) in providers_snapshot.iter() {
                    let provider_name = name.clone();
                    let base_url = provider.base_url.clone();
                    
                    let result = Self::check_provider_health(&provider_name, &base_url, timeout, max_retries, retry_delay_ms).await;
                    health_checks.push(result);
                    
                    if health_checks.last().map(|r| r.is_healthy).unwrap_or(false) {
                        all_unhealthy = false;
                    }
                }

                drop(providers_snapshot);
                
                {
                    let mut status = health_status_arc.write().await;
                    *status = health_checks.clone();
                }

                if all_unhealthy {
                    tracing::warn!("All providers are unhealthy!");
                    let mut active = active_arc.write().await;
                    *active = None;
                } else {
                    if let Some(first_healthy) = health_checks.iter().find(|r| r.is_healthy) {
                        let mut active = active_arc.write().await;
                        if *active != Some(first_healthy.provider_name.clone()) {
                            tracing::info!("Switching to backup provider: {}", first_healthy.provider_name);
                            *active = Some(first_healthy.provider_name.clone());
                        }
                    }
                }
            }
        });

        tracing::info!("Health check started, interval: {} seconds", self.config.health_check_interval_secs);
    }

    async fn check_provider_health(
        name: &str,
        base_url: &str,
        timeout: Duration,
        max_retries: u32,
        retry_delay_ms: u64,
    ) -> HealthCheckResult {
        let mut last_error = None;

        for attempt in 0..max_retries {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(retry_delay_ms)).await;
            }

            let start = std::time::Instant::now();
            let health_url = format!("{}/health", base_url.trim_end_matches('/'));

            match tokio::time::timeout(
                timeout,
                reqwest::get(&health_url)
            ).await {
                Ok(Ok(response)) => {
                    let latency = start.elapsed().as_millis() as u64;
                    if response.status().is_success() {
                        tracing::debug!("Provider {} health check succeeded, latency: {}ms", name, latency);
                        return HealthCheckResult {
                            provider_name: name.to_string(),
                            is_healthy: true,
                            latency_ms: Some(latency),
                            error_message: None,
                            last_check: chrono::Utc::now(),
                        };
                    } else {
                        last_error = Some(format!("HTTP {}", response.status()));
                    }
                }
                Ok(Err(e)) => {
                    last_error = Some(e.to_string());
                }
                Err(_) => {
                    last_error = Some("Health check timeout".to_string());
                }
            }
        }

        tracing::warn!("Provider {} health check failed: {:?}", name, last_error);
        HealthCheckResult {
            provider_name: name.to_string(),
            is_healthy: false,
            latency_ms: None,
            error_message: last_error,
            last_check: chrono::Utc::now(),
        }
    }

    pub async fn get_active_provider(&self) -> Option<(String, ProviderConfig)> {
        let active_name = self.active_provider.read().await;
        if let Some(name) = active_name.clone() {
            let providers = self.providers.read().await;
            providers.iter()
                .find(|(n, _)| n == &name)
                .map(|(n, p)| (n.clone(), p.clone()))
        } else {
            None
        }
    }

    pub async fn get_health_status(&self) -> Vec<HealthCheckResult> {
        self.health_status.read().await.clone()
    }

    pub async fn manual_switch(&self, provider_name: &str) -> Result<(), String> {
        let providers = self.providers.read().await;
        if !providers.iter().any(|(name, _)| name == provider_name) {
            return Err(format!("Provider not found: {}", provider_name));
        }
        drop(providers);
        let mut active = self.active_provider.write().await;
        *active = Some(provider_name.to_string());
        tracing::info!("Manual switch to provider: {}", provider_name);
        Ok(())
    }
}

pub type FailoverState = Arc<FailoverManager>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_failover_config_default() {
        let config = FailoverConfig::default();
        assert!(config.enabled);
        assert_eq!(config.health_check_interval_secs, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_health_check_result_serialization() {
        let result = HealthCheckResult {
            provider_name: "test".to_string(),
            is_healthy: true,
            latency_ms: Some(100),
            error_message: None,
            last_check: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"is_healthy\":true"));
    }
}
