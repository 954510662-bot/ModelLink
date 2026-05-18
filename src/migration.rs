use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigVersion {
    pub version: String,
    pub migrated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ConfigVersion {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            migrated_at: None,
        }
    }

    pub fn with_migration(version: &str) -> Self {
        Self {
            version: version.to_string(),
            migrated_at: Some(chrono::Utc::now()),
        }
    }
}

pub struct ConfigMigrator {
    migrations: HashMap<String, Box<dyn Fn(serde_yaml::Value) -> anyhow::Result<serde_yaml::Value> + Send + Sync>>,
}

impl ConfigMigrator {
    pub fn new() -> Self {
        let mut migrator = Self {
            migrations: HashMap::new(),
        };
        
        migrator.register_migration("0.1.0", "0.2.0", Self::migrate_v1_to_v2);
        migrator.register_migration("0.2.0", "0.3.0", Self::migrate_v2_to_v3);
        
        migrator
    }

    pub fn register_migration<F>(&mut self, from: &str, to: &str, func: F)
    where
        F: Fn(serde_yaml::Value) -> anyhow::Result<serde_yaml::Value> + Send + Sync + 'static,
    {
        let key = format!("{}->{}", from, to);
        self.migrations.insert(key, Box::new(func));
        tracing::debug!("已注册迁移路径: {} -> {}", from, to);
    }

    pub fn get_current_version(config: &serde_yaml::Value) -> String {
        config
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("0.1.0")
            .to_string()
    }

    pub fn set_version(config: &mut serde_yaml::Value, version: &str) {
        if let Some(obj) = config.as_mapping_mut() {
            obj.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::String(version.to_string()),
            );
        }
    }

    pub fn migrate(&self, config: serde_yaml::Value, from_version: &str, to_version: &str) -> anyhow::Result<serde_yaml::Value> {
        let key = format!("{}->{}", from_version, to_version);
        
        if let Some(migration) = self.migrations.get(&key) {
            tracing::info!("执行配置迁移: {} -> {}", from_version, to_version);
            let result = migration(config)?;
            Ok(result)
        } else {
            Err(anyhow::anyhow!(
                "未找到迁移路径: {} -> {}",
                from_version,
                to_version
            ))
        }
    }

    pub fn migrate_to_latest(&self, config: serde_yaml::Value) -> anyhow::Result<serde_yaml::Value> {
        let current = Self::get_current_version(&config);
        let latest = "0.3.0";
        
        if current == latest {
            tracing::debug!("配置已是最新版本: {}", latest);
            return Ok(config);
        }

        let mut result = config;
        let mut from = current.clone();
        
        while from != latest {
            let key = format!("{}->{}", from, Self::next_version(&from));
            if let Some(migration) = self.migrations.get(&key) {
                tracing::info!("执行迁移: {} -> {}", from, Self::next_version(&from));
                result = migration(result)?;
                from = Self::next_version(&from);
            } else {
                return Err(anyhow::anyhow!(
                    "无法找到从 {} 开始的迁移路径",
                    from
                ));
            }
        }

        tracing::info!("配置已迁移到最新版本: {}", latest);
        Ok(result)
    }

    fn next_version(version: &str) -> String {
        match version {
            "0.1.0" => "0.2.0".to_string(),
            "0.2.0" => "0.3.0".to_string(),
            _ => version.to_string(),
        }
    }

    fn migrate_v1_to_v2(config: serde_yaml::Value) -> anyhow::Result<serde_yaml::Value> {
        let mut result = config;
        
        if let Some(obj) = result.as_mapping_mut() {
            obj.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::String("0.2.0".to_string()),
            );
            
            if let Some(providers) = obj.get_mut("providers") {
                if let Some(provider_map) = providers.as_mapping_mut() {
                    for (_, provider) in provider_map.iter_mut() {
                        if let Some(provider_obj) = provider.as_mapping_mut() {
                            provider_obj.insert(
                                serde_yaml::Value::String("timeout".to_string()),
                                serde_yaml::Value::Number(30.into()),
                            );
                        }
                    }
                }
            }
        }
        
        tracing::info!("v1 到 v2 迁移完成");
        Ok(result)
    }

    fn migrate_v2_to_v3(config: serde_yaml::Value) -> anyhow::Result<serde_yaml::Value> {
        let mut result = config;
        
        if let Some(obj) = result.as_mapping_mut() {
            obj.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::String("0.3.0".to_string()),
            );
            
            if !obj.contains_key(&serde_yaml::Value::String("failover".to_string())) {
                let failover = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
                obj.insert(
                    serde_yaml::Value::String("failover".to_string()),
                    failover,
                );
            }
            
            if let Some(failover) = obj.get_mut("failover") {
                if let Some(failover_obj) = failover.as_mapping_mut() {
                    failover_obj.insert(
                        serde_yaml::Value::String("enabled".to_string()),
                        serde_yaml::Value::Bool(true),
                    );
                    failover_obj.insert(
                        serde_yaml::Value::String("health_check_interval".to_string()),
                        serde_yaml::Value::Number(30.into()),
                    );
                }
            }
        }
        
        tracing::info!("v2 到 v3 迁移完成");
        Ok(result)
    }

    pub fn validate_version(&self, version: &str) -> bool {
        matches!(version, "0.1.0" | "0.2.0" | "0.3.0")
    }

    pub fn get_supported_versions(&self) -> Vec<String> {
        vec![
            "0.1.0".to_string(),
            "0.2.0".to_string(),
            "0.3.0".to_string(),
        ]
    }
}

impl Default for ConfigMigrator {
    fn default() -> Self {
        Self::new()
    }
}

pub type MigratorState = std::sync::Arc<ConfigMigrator>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let config = serde_yaml::Value::Mapping({
            let mut map = serde_yaml::Mapping::new();
            map.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::String("0.1.0".to_string()),
            );
            map
        });

        let version = ConfigMigrator::get_current_version(&config);
        assert_eq!(version, "0.1.0");
    }

    #[test]
    fn test_version_validation() {
        let migrator = ConfigMigrator::new();
        
        assert!(migrator.validate_version("0.1.0"));
        assert!(migrator.validate_version("0.2.0"));
        assert!(migrator.validate_version("0.3.0"));
        assert!(!migrator.validate_version("0.0.1"));
    }

    #[test]
    fn test_migration_v1_to_v2() {
        let migrator = ConfigMigrator::new();
        
        let config = serde_yaml::Value::Mapping({
            let mut map = serde_yaml::Mapping::new();
            map.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::String("0.1.0".to_string()),
            );
            let providers = serde_yaml::Value::Mapping({
                let mut p = serde_yaml::Mapping::new();
                p.insert(
                    serde_yaml::Value::String("test".to_string()),
                    serde_yaml::Value::Mapping(serde_yaml::Mapping::new()),
                );
                p
            });
            map.insert(serde_yaml::Value::String("providers".to_string()), providers);
            map
        });

        let result = migrator.migrate(config, "0.1.0", "0.2.0").unwrap();
        assert_eq!(ConfigMigrator::get_current_version(&result), "0.2.0");
    }

    #[test]
    fn test_migration_to_latest() {
        let migrator = ConfigMigrator::new();
        
        let config = serde_yaml::Value::Mapping({
            let mut map = serde_yaml::Mapping::new();
            map.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::String("0.1.0".to_string()),
            );
            map
        });

        let result = migrator.migrate_to_latest(config).unwrap();
        assert_eq!(ConfigMigrator::get_current_version(&result), "0.3.0");
    }
}
