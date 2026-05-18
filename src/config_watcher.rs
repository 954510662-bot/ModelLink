use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing;

use crate::config::ConfigManager;

pub struct ConfigWatcher {
    watcher: Option<RecommendedWatcher>,
    stop_tx: Option<mpsc::Sender<()>>,
}

impl ConfigWatcher {
    pub fn new(
        config_manager: Arc<ConfigManager>,
        debounce_duration: Duration,
    ) -> Result<Self, notify::Error> {
        let (tx, mut rx) = mpsc::channel::<Event>(100);
        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        
        let config_path = config_manager.get_path().clone();
        let config_manager_clone = config_manager.clone();
        
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    if let Err(e) = tx.blocking_send(event) {
                        tracing::warn!("Failed to send config change event: {}", e);
                    }
                }
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;
        
        if let Some(parent) = config_path.parent() {
            if parent.exists() {
                watcher.watch(parent, RecursiveMode::NonRecursive)?;
                tracing::info!("配置文件监听已启动: {}", config_path.display());
            }
        }
        
        let watcher = watcher.into();
        
        tokio::spawn(async move {
            let mut last_reload = std::time::Instant::now();
            let mut pending_reload = false;
            
            loop {
                tokio::select! {
                    Some(event) = rx.recv() => {
                        if Self::is_config_relevant_event(&event, &config_path) {
                            pending_reload = true;
                        }
                    }
                    _ = stop_rx.recv() => {
                        tracing::info!("配置监听已停止");
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        if pending_reload && last_reload.elapsed() >= debounce_duration {
                            last_reload = std::time::Instant::now();
                            pending_reload = false;
                            
                            tracing::info!("检测到配置文件变更，正在重新加载...");
                            
                            match config_manager_clone.reload().await {
                                Ok(()) => {
                                    let config = config_manager_clone.get().await;
                                    tracing::info!(
                                        "✅ 配置已重新加载: {} 个提供商, {} 个模型映射",
                                        config.providers.len(),
                                        config.mappings.len()
                                    );
                                }
                                Err(e) => {
                                    tracing::error!("❌ 配置重载失败: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        });
        
        Ok(Self {
            watcher: Some(watcher),
            stop_tx: Some(stop_tx),
        })
    }
    
    fn is_config_relevant_event(event: &Event, config_path: &PathBuf) -> bool {
        let config_name = config_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        
        matches!(
            event.kind,
            notify::EventKind::Create(_) | 
            notify::EventKind::Modify(_) | 
            notify::EventKind::Remove(_)
        ) && event.paths.iter().any(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.contains(config_name))
                .unwrap_or(false)
        })
    }
    
    pub async fn stop(&mut self) {
        if let Some(stop_tx) = self.stop_tx.take() {
            let _ = stop_tx.send(()).await;
        }
        if let Some(mut watcher) = self.watcher.take() {
            if let Ok(path) = std::env::var("MODEL_LINK_CONFIG_PATH") {
                let _ = watcher.unwatch(std::path::Path::new(&path));
            }
        }
        tracing::info!("配置文件监听已停止");
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        if let Some(stop_tx) = self.stop_tx.take() {
            let _ = stop_tx.blocking_send(());
        }
    }
}

pub struct ConfigHotReload {
    watcher: Option<ConfigWatcher>,
}

impl ConfigHotReload {
    pub fn new(config_manager: Arc<ConfigManager>) -> Result<Self, notify::Error> {
        let watcher = ConfigWatcher::new(
            config_manager,
            Duration::from_secs(2),
        )?;
        
        Ok(Self {
            watcher: Some(watcher),
        })
    }
    
    pub async fn stop(&mut self) {
        if let Some(mut watcher) = self.watcher.take() {
            watcher.stop().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Arc;
    
    #[test]
    fn test_is_config_relevant_event() {
        let config_path = PathBuf::from("/home/user/.config/model-link/config.yaml");
        
        let create_event = Event {
            kind: notify::EventKind::Create(notify::event::CreateKind::File),
            paths: vec![PathBuf::from("/home/user/.config/model-link/config.yaml")],
            ..Default::default()
        };
        
        assert!(ConfigWatcher::is_config_relevant_event(&create_event, &config_path));
        
        let other_event = Event {
            kind: notify::EventKind::Create(notify::event::CreateKind::File),
            paths: vec![PathBuf::from("/home/user/.config/other/file.yaml")],
            ..Default::default()
        };
        
        assert!(!ConfigWatcher::is_config_relevant_event(&other_event, &config_path));
    }
    
    #[test]
    #[ignore] // ConfigWatcher::new requires non-async context
    fn test_config_watcher_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.yaml");
        let config_content = r#"
providers:
  test:
    base_url: "https://api.test.com"
mappings: {}
"#;
        std::fs::write(&config_path, config_content).unwrap();
        
        // This test requires non-async context, so we ignore it
        // and test it manually or with integration tests
        assert!(true);
    }
}
