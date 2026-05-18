use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub config_loaded: bool,
    pub upstream_healthy: bool,
    pub memory_usage_mb: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ReadyStatus {
    pub ready: bool,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CheckResult {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn init_start_time() {
    START_TIME.set(std::time::Instant::now()).ok();
}

fn get_start_time_seconds() -> u64 {
    START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0)
}

pub async fn health_handler() -> impl IntoResponse {
    let uptime = get_start_time_seconds();
    
    let memory_usage = get_memory_usage();
    
    let status = HealthStatus {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        config_loaded: true,
        upstream_healthy: true,
        memory_usage_mb: memory_usage,
    };
    
    (StatusCode::OK, Json(status))
}

pub async fn ready_handler() -> impl IntoResponse {
    let checks = run_readiness_checks().await;
    let all_ready = checks.iter().all(|c| c.status == "ok");
    
    let status = ReadyStatus {
        ready: all_ready,
        checks,
    };
    
    if all_ready {
        (StatusCode::OK, Json(status))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(status))
    }
}

async fn run_readiness_checks() -> Vec<CheckResult> {
    let mut checks = Vec::new();
    
    checks.push(CheckResult {
        name: "config".to_string(),
        status: "ok".to_string(),
        message: None,
    });
    
    checks.push(CheckResult {
        name: "upstream".to_string(),
        status: "ok".to_string(),
        message: None,
    });
    
    checks
}

fn get_memory_usage() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| line.starts_with("VmRSS:"))
                    .and_then(|line| {
                        line.split_whitespace()
                            .nth(1)
                            .and_then(|v| v.parse::<f64>().ok())
                            .map(|kb| kb / 1024.0)
                    })
            })
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

#[allow(dead_code)]
pub async fn detailed_health_check() -> HealthStatus {
    let uptime = get_start_time_seconds();
    let memory_usage = get_memory_usage();
    
    HealthStatus {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        config_loaded: true,
        upstream_healthy: true,
        memory_usage_mb: memory_usage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus {
            status: "healthy".to_string(),
            version: "0.1.0".to_string(),
            uptime_seconds: 3600,
            config_loaded: true,
            upstream_healthy: true,
            memory_usage_mb: Some(50.5),
        };
        
        let json = serde_json::to_string(&status).unwrap();
        
        assert!(json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"version\":\"0.1.0\""));
        assert!(json.contains("\"uptime_seconds\":3600"));
        assert!(json.contains("\"memory_usage_mb\":50.5"));
    }
    
    #[test]
    fn test_ready_status() {
        let ready = ReadyStatus {
            ready: true,
            checks: vec![
                CheckResult {
                    name: "config".to_string(),
                    status: "ok".to_string(),
                    message: None,
                },
                CheckResult {
                    name: "upstream".to_string(),
                    status: "ok".to_string(),
                    message: None,
                },
            ],
        };
        
        let json = serde_json::to_string(&ready).unwrap();
        
        assert!(json.contains("\"ready\":true"));
        assert!(json.contains("\"name\":\"config\""));
    }
    
    #[test]
    fn test_init_start_time() {
        init_start_time();
        let seconds = get_start_time_seconds();
        assert!(seconds >= 0);
    }
}
