use std::path::{Path, PathBuf};
use tokio::fs;
use serde::{Deserialize, Serialize};
use tracing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub size_bytes: u64,
    pub checksum: Option<String>,
}

pub struct ConfigBackup {
    backup_dir: PathBuf,
    max_backups: usize,
}

impl ConfigBackup {
    pub fn new(backup_dir: PathBuf, max_backups: usize) -> Self {
        Self {
            backup_dir,
            max_backups,
        }
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.backup_dir).await?;
        tracing::info!("备份目录已创建: {}", self.backup_dir.display());
        Ok(())
    }

    pub async fn create_backup(&self, config_path: &Path) -> anyhow::Result<BackupInfo> {
        if !config_path.exists() {
            return Err(anyhow::anyhow!("配置文件不存在: {}", config_path.display()));
        }

        let content = fs::read(config_path).await?;
        let timestamp = chrono::Utc::now();
        let size_bytes = content.len() as u64;

        let backup_name = format!(
            "config_backup_{}.yaml",
            timestamp.format("%Y%m%d_%H%M%S")
        );
        let backup_path = self.backup_dir.join(&backup_name);

        fs::write(&backup_path, &content).await?;

        let checksum = Some(self.calculate_checksum(&content));

        let info = BackupInfo {
            path: backup_path.clone(),
            timestamp,
            size_bytes,
            checksum,
        };

        tracing::info!(
            "配置文件已备份: {} ({} bytes)",
            backup_path.display(),
            size_bytes
        );

        self.cleanup_old_backups().await?;

        Ok(info)
    }

    pub async fn restore_backup(&self, backup_name: &str) -> anyhow::Result<PathBuf> {
        let backup_path = self.backup_dir.join(backup_name);
        
        if !backup_path.exists() {
            return Err(anyhow::anyhow!("备份文件不存在: {}", backup_path.display()));
        }

        let content = fs::read(&backup_path).await?;
        
        let restore_path = PathBuf::from("config.yaml");
        fs::write(&restore_path, content).await?;

        tracing::info!("配置已恢复: {}", restore_path.display());
        
        Ok(restore_path)
    }

    pub async fn list_backups(&self) -> anyhow::Result<Vec<BackupInfo>> {
        let mut entries = fs::read_dir(&self.backup_dir).await?;
        let mut backups = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map(|e| e == "yaml").unwrap_or(false) {
                let metadata = fs::metadata(&path).await?;
                let content = fs::read(&path).await?;
                
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                
                let timestamp = Self::parse_timestamp_from_filename(&filename)
                    .unwrap_or_else(chrono::Utc::now);

                backups.push(BackupInfo {
                    path: path.clone(),
                    timestamp,
                    size_bytes: metadata.len(),
                    checksum: Some(self.calculate_checksum(&content)),
                });
            }
        }

        backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(backups)
    }

    pub async fn delete_backup(&self, backup_name: &str) -> anyhow::Result<()> {
        let backup_path = self.backup_dir.join(backup_name);
        
        if !backup_path.exists() {
            return Err(anyhow::anyhow!("备份文件不存在: {}", backup_path.display()));
        }

        fs::remove_file(&backup_path).await?;
        tracing::info!("备份已删除: {}", backup_path.display());

        Ok(())
    }

    async fn cleanup_old_backups(&self) -> anyhow::Result<()> {
        let backups = self.list_backups().await?;
        
        if backups.len() > self.max_backups {
            let to_delete = backups[self.max_backups..].to_vec();
            for backup in to_delete {
                if let Err(e) = fs::remove_file(&backup.path).await {
                    tracing::warn!("删除旧备份失败: {}", e);
                } else {
                    tracing::debug!("已删除旧备份: {}", backup.path.display());
                }
            }
            tracing::info!("旧备份已清理，保留最近 {} 个备份", self.max_backups);
        }

        Ok(())
    }

    fn calculate_checksum(&self, content: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn parse_timestamp_from_filename(filename: &str) -> Option<chrono::DateTime<chrono::Utc>> {
        let parts: Vec<&str> = filename.split('_').collect();
        if parts.len() >= 4 {
            let date_part = parts.get(2)?;
            let time_part = parts.get(3)?.trim_end_matches(".yaml");
            
            let combined = format!("{}_{}", date_part, time_part);
            chrono::NaiveDateTime::parse_from_str(&combined, "%Y%m%d_%H%M%S")
                .ok()
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
        } else {
            None
        }
    }

    pub async fn get_latest_backup(&self) -> anyhow::Result<Option<BackupInfo>> {
        let backups = self.list_backups().await?;
        Ok(backups.into_iter().next())
    }

    pub async fn verify_backup(&self, backup_name: &str) -> anyhow::Result<bool> {
        let backup_path = self.backup_dir.join(backup_name);
        
        if !backup_path.exists() {
            return Ok(false);
        }

        let content = fs::read(&backup_path).await?;
        let expected_checksum = self.calculate_checksum(&content);

        if let Some(backups) = self.list_backups().await?.into_iter().find(|b| b.path == backup_path) {
            Ok(backups.checksum.as_ref() == Some(&expected_checksum))
        } else {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_backup_creation() {
        let temp_dir = tempdir().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let backup = ConfigBackup::new(backup_dir.clone(), 5);
        
        backup.init().await.unwrap();
        
        let config_path = temp_dir.path().join("config.yaml");
        fs::write(&config_path, "test: true").await.unwrap();
        
        let info = backup.create_backup(&config_path).await.unwrap();
        assert!(info.path.exists());
        assert_eq!(info.size_bytes, 10);
    }

    #[tokio::test]
    async fn test_list_backups() {
        let temp_dir = tempdir().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        let backup = ConfigBackup::new(backup_dir, 5);
        
        backup.init().await.unwrap();
        
        let backups = backup.list_backups().await.unwrap();
        assert_eq!(backups.len(), 0);
    }

    #[test]
    fn test_timestamp_parsing() {
        let filename = "config_backup_20240115_143022.yaml";
        let result = ConfigBackup::parse_timestamp_from_filename(filename);
        
        assert!(result.is_some(), "Failed to parse: {}", filename);
        if let Some(dt) = result {
            assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");
        }
    }
}
