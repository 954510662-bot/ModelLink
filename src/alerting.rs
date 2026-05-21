use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub title: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub channels: Vec<AlertChannel>,
    pub rules: Vec<AlertRule>,
    pub cooldown_seconds: u64,
    pub max_alerts_per_hour: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    pub name: String,
    pub channel_type: AlertChannelType,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannelType {
    Email,
    Webhook,
    Slack,
    PagerDuty,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: String,
    pub threshold: f64,
    pub duration_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertStats {
    pub total_alerts: u64,
    pub active_alerts: u64,
    pub resolved_alerts: u64,
    pub critical_count: u64,
    pub high_count: u64,
    pub medium_count: u64,
    pub low_count: u64,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: Vec::new(),
            rules: vec![
                AlertRule {
                    name: "High Error Rate".to_string(),
                    condition: AlertCondition {
                        metric: "error_rate".to_string(),
                        operator: ">".to_string(),
                        threshold: 0.05,
                        duration_seconds: 60,
                    },
                    severity: AlertSeverity::High,
                    enabled: true,
                },
                AlertRule {
                    name: "High Latency".to_string(),
                    condition: AlertCondition {
                        metric: "latency_p95".to_string(),
                        operator: ">".to_string(),
                        threshold: 1000.0,
                        duration_seconds: 300,
                    },
                    severity: AlertSeverity::Medium,
                    enabled: true,
                },
            ],
            cooldown_seconds: 300,
            max_alerts_per_hour: 100,
        }
    }
}

pub struct AlertManagerState {
    pub alerts: VecDeque<Alert>,
    pub stats: AlertStats,
}

impl Default for AlertManagerState {
    fn default() -> Self {
        Self {
            alerts: VecDeque::new(),
            stats: AlertStats {
                total_alerts: 0,
                active_alerts: 0,
                resolved_alerts: 0,
                critical_count: 0,
                high_count: 0,
                medium_count: 0,
                low_count: 0,
            },
        }
    }
}

pub struct AlertManager {
    config: Arc<RwLock<AlertConfig>>,
    state: Arc<RwLock<AlertManagerState>>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(AlertConfig::default())),
            state: Arc::new(RwLock::new(AlertManagerState::default())),
        }
    }

    pub async fn initialize(&self, config: AlertConfig) {
        let mut cfg = self.config.write().await;
        *cfg = config;
    }

    pub async fn get_config(&self) -> AlertConfig {
        self.config.read().await.clone()
    }

    pub async fn get_state(&self) -> AlertManagerState {
        self.state.read().await.clone()
    }

    pub async fn create_alert(&self, title: String, message: String, severity: AlertSeverity, source: String) -> Alert {
        let alert = Alert {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            message,
            severity,
            source,
            timestamp: Utc::now(),
            resolved: false,
            metadata: serde_json::json!({}),
        };

        let mut state = self.state.write().await;
        
        if state.alerts.len() >= 1000 {
            state.alerts.pop_front();
        }
        
        state.alerts.push_back(alert.clone());
        state.stats.total_alerts += 1;
        state.stats.active_alerts += 1;
        
        match alert.severity {
            AlertSeverity::Critical => state.stats.critical_count += 1,
            AlertSeverity::High => state.stats.high_count += 1,
            AlertSeverity::Medium => state.stats.medium_count += 1,
            AlertSeverity::Low => state.stats.low_count += 1,
            AlertSeverity::Info => {}
        }
        
        alert
    }

    pub async fn resolve_alert(&self, alert_id: &str) -> Result<(), String> {
        let mut state = self.state.write().await;
        
        if let Some(alert) = state.alerts.iter_mut().find(|a| a.id == alert_id) {
            if !alert.resolved {
                alert.resolved = true;
                state.stats.active_alerts -= 1;
                state.stats.resolved_alerts += 1;
            }
            Ok(())
        } else {
            Err(format!("Alert with id {} not found", alert_id))
        }
    }

    pub async fn get_active_alerts(&self) -> Vec<Alert> {
        let state = self.state.read().await;
        state.alerts.iter().filter(|a| !a.resolved).cloned().collect()
    }

    pub async fn get_alert_history(&self, limit: usize) -> Vec<Alert> {
        let state = self.state.read().await;
        state.alerts.iter().rev().take(limit).cloned().collect()
    }

    pub async fn get_stats(&self) -> AlertStats {
        let state = self.state.read().await;
        state.stats.clone()
    }

    pub async fn check_rules(&self, metrics: &serde_json::Value) -> Vec<Alert> {
        let config = self.config.read().await;
        let mut alerts = Vec::new();

        for rule in &config.rules {
            if !rule.enabled {
                continue;
            }

            let metric_value = metrics.get(&rule.condition.metric)
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);

            let should_alert = match rule.condition.operator.as_str() {
                ">" => metric_value > rule.condition.threshold,
                ">=" => metric_value >= rule.condition.threshold,
                "<" => metric_value < rule.condition.threshold,
                "<=" => metric_value <= rule.condition.threshold,
                "==" => (metric_value - rule.condition.threshold).abs() < f64::EPSILON,
                _ => false,
            };

            if should_alert {
                let alert = self.create_alert(
                    rule.name.clone(),
                    format!("Metric {} {} {} (current: {})", 
                        rule.condition.metric,
                        rule.condition.operator,
                        rule.condition.threshold,
                        metric_value
                    ),
                    rule.severity.clone(),
                    "system".to_string(),
                ).await;
                alerts.push(alert);
            }
        }

        alerts
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}
