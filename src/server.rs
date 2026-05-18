use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::signal;

use crate::{config::ConfigManager, config_watcher::ConfigHotReload, proxy::create_router};

pub async fn start_server(
    config_path: Option<PathBuf>,
    host: Option<String>,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let config_manager = Arc::new(ConfigManager::new(config_path).await?);
    let config = config_manager.get().await;
    
    let listen_host = host.unwrap_or_else(|| config.server.host.clone());
    let listen_port = port.unwrap_or(config.server.port);
    
    let addr: SocketAddr = format!("{}:{}", listen_host, listen_port).parse().unwrap();
    
    let config_manager_for_watcher = config_manager.clone();
    let mut hot_reload = ConfigHotReload::new(config_manager_for_watcher)?;
    
    let router = create_router(config_manager.clone()).await;
    
    tracing::info!("🚀 ModelLink 服务启动");
    tracing::info!("监听地址: http://{}", addr);
    tracing::info!("配置文件: {}", config_manager.get_path().display());
    tracing::info!("已加载 {} 个提供商, {} 个模型映射", 
        config.providers.len(), 
        config.mappings.len()
    );
    tracing::info!("✅ 配置热加载已启用，配置文件变更将在 2 秒内自动应用");
    
    let listener = TcpListener::bind(&addr).await?;
    
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    hot_reload.stop().await;
    
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("🛑 正在关闭服务...");
}
