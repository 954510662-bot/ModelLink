use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use clap_complete::{Shell, generate, Generator};
#[cfg(feature = "update")]
use self_update::cargo_crate_version;

#[derive(Parser)]
#[command(name = "model-link")]
#[command(about = "A local proxy that allows AI coding tools to use any third-party model")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(short, long)]
        config: Option<PathBuf>,
        
        #[arg(long, visible_short_alias = 'H')]
        host: Option<String>,
        
        #[arg(short, long)]
        port: Option<u16>,
    },
    
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    
    Completions {
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
    
    #[cfg(feature = "update")]
    Update {
        #[arg(short, long)]
        check: bool,
        
        #[arg(short, long)]
        yes: bool,
    },
    
    Doctor,
    
    Version,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    Init {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    Validate {
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    Path,
}

pub async fn handle_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Start { config, host, port } => {
            crate::server::start_server(config, host, port).await
        }
        Commands::Config { command } => handle_config_command(command).await,
        Commands::Completions { shell } => generate_completions(shell),
        #[cfg(feature = "update")]
        Commands::Update { check, yes } => handle_update(check, yes).await,
        Commands::Doctor => run_doctor().await,
        Commands::Version => print_version(),
    }
}

fn generate_completions(shell: Shell) -> anyhow::Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}

async fn handle_config_command(command: ConfigCommands) -> anyhow::Result<()> {
    match command {
        ConfigCommands::Init { output } => init_config(output).await,
        ConfigCommands::Validate { config } => validate_config(config).await,
        ConfigCommands::Path => print_config_path(),
    }
}

async fn init_config(output: Option<PathBuf>) -> anyhow::Result<()> {
    let path = output.unwrap_or_else(|| {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("model-link")
            .join("config.yaml")
    });
    
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    
    let content = include_str!("../config-template.yaml");
    tokio::fs::write(&path, content).await?;
    
    println!("✅ 配置文件已生成: {}", path.display());
    Ok(())
}

async fn validate_config(config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let config = crate::config::ConfigManager::new(config_path).await?;
    println!("✅ 配置文件验证通过");
    println!("配置路径: {}", config.get_path().display());
    
    let cfg = config.get().await;
    println!("服务器地址: {}:{}", cfg.server.host, cfg.server.port);
    println!("提供商数量: {}", cfg.providers.len());
    println!("模型映射数量: {}", cfg.mappings.len());
    
    Ok(())
}

fn print_config_path() -> anyhow::Result<()> {
    let path = crate::config::ConfigManager::default_config_path();
    println!("{}", path.display());
    Ok(())
}

async fn run_doctor() -> anyhow::Result<()> {
    println!("🧙 ModelLink 诊断工具\n");
    
    println!("检查 Rust 环境...");
    match rustc_version::version() {
        Ok(version) => println!("  ✅ Rust 版本: {}", version),
        Err(_) => println!("  ⚠️  无法获取 Rust 版本"),
    }
    
    println!("检查配置文件...");
    let config_path = crate::config::ConfigManager::default_config_path();
    if config_path.exists() {
        println!("  ✅ 配置文件存在: {}", config_path.display());
    } else {
        println!("  ⚠️  配置文件不存在，将使用默认配置");
    }
    
    println!("检查端口占用...");
    let port = 9191;
    if is_port_available(port).await {
        println!("  ✅ 端口 {} 可用", port);
    } else {
        println!("  ⚠️  端口 {} 已被占用", port);
    }
    
    println!("\n✅ 诊断完成");
    Ok(())
}

async fn is_port_available(port: u16) -> bool {
    match tokio::net::TcpListener::bind(("127.0.0.1", port)).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn print_version() -> anyhow::Result<()> {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Ok(())
}

#[cfg(feature = "update")]
async fn handle_update(check_only: bool, no_confirm: bool) -> anyhow::Result<()> {
    use self_update::backends::github::Update;
    use self_update::update::Release;
    
    println!("🔄 ModelLink 自动更新\n");
    
    let current_version = cargo_crate_version!();
    println!("当前版本: v{}", current_version);
    
    let release = Update::configure()
        .repo_owner("954510662-bot")
        .repo_name("ModelLink")
        .bin_name("model-link")
        .show_download_progress(true)
        .current_version(current_version)
        .build()?;
    
    println!("检查更新...");
    let latest_release = release.get_latest_release()?;
    
    if latest_release.version == current_version {
        println!("✅ 已是最新版本！");
        return Ok(());
    }
    
    println!("发现新版本: v{}", latest_release.version);
    println!("发布日期: {}", latest_release.date);
    println!("\n发布说明:\n{}", latest_release.body);
    
    if check_only {
        return Ok(());
    }
    
    if !no_confirm {
        println!("\n是否继续更新? (y/N)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("更新已取消");
            return Ok(());
        }
    }
    
    println!("\n开始更新...");
    let status = release.update()?;
    
    match status {
        self_update::Status::UpToDate => {
            println!("✅ 已是最新版本！");
        }
        self_update::Status::Updated(v) => {
            println!("✅ 更新成功！已升级到 v{}", v);
        }
    }
    
    Ok(())
}
