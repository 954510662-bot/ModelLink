mod audit;
mod backup;
mod cli;
mod config;
mod config_watcher;
mod errors;
mod failover;
mod health;
mod http_client;
mod metrics;
mod migration;
mod mock;
mod models;
mod proxy;
mod rate_limit;
mod server;
mod stream;
mod translator;
mod validation;
mod utils;
mod wizard;

pub use audit::{AuditEntry, AuditLogger, MetricsCollector, MetricsSnapshot};
pub use backup::{BackupInfo, ConfigBackup};
pub use cli::{Cli, handle_cli};
pub use config::{Config, ConfigManager, ModelCapabilities, ModelCapabilityDB, ProviderConfig, ServerConfig};
pub use config_watcher::{ConfigHotReload, ConfigWatcher};
pub use errors::{ModelLinkError, Result};
pub use failover::{FailoverConfig, FailoverManager, FailoverState, HealthCheckResult};
pub use health::{HealthStatus, health_handler, ready_handler, init_start_time};
pub use http_client::{HttpClientConfig, HttpClientPool, HttpClientPoolState};
pub use migration::{ConfigMigrator, ConfigVersion, MigratorState};
pub use metrics::{MetricsRecorder, MetricsState, RecorderState, create_metrics_router};
pub use mock::{MockMode, MockResponse, MockServer};
pub use models::*;
pub use proxy::create_router;
pub use rate_limit::{RateLimitConfig, RateLimiter, RateLimitState, rate_limit_middleware};
pub use server::start_server;
pub use stream::*;
pub use translator::{ParameterTranslator, TranslateResult, translate_request_for_model};
pub use utils::{convert_headers, sanitize_log_input, generate_request_id};
pub use validation::RequestValidator;
pub use wizard::{ConfigWizard, WizardAnswers, ProviderType, QuickSetup};

#[cfg(test)]
mod tests {
    pub use super::*;
}
