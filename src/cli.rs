use clap::{Parser, Subcommand, ValueEnum, CommandFactory};
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
    
    println!("Config file generated: {}", path.display());
    Ok(())
}

async fn validate_config(config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let config = crate::config::ConfigManager::new(config_path).await?;
    println!("Config file validation passed");
    println!("Config path: {}", config.get_path().display());
    
    let cfg = config.get().await;
    println!("Server address: {}:{}", cfg.server.host, cfg.server.port);
    println!("Provider count: {}", cfg.providers.len());
    println!("Model mapping count: {}", cfg.mappings.len());
    
    Ok(())
}

fn print_config_path() -> anyhow::Result<()> {
    let path = crate::config::ConfigManager::default_config_path();
    println!("{}", path.display());
    Ok(())
}

async fn run_doctor() -> anyhow::Result<()> {
    println!("ModelLink Diagnostics\n");
    
    println!("Checking Rust environment...");
    match rustc_version::version() {
        Ok(version) => println!("  [OK] Rust version: {}", version),
        Err(_) => println!("  [WARN] Cannot get Rust version"),
    }
    
    println!("Checking config file...");
    let config_path = crate::config::ConfigManager::default_config_path();
    if config_path.exists() {
        println!("  [OK] Config file exists: {}", config_path.display());
    } else {
        println!("  [WARN] Config file not found, will use default config");
    }
    
    println!("Checking port availability...");
    let port = 9191;
    if is_port_available(port).await {
        println!("  [OK] Port {} is available", port);
    } else {
        println!("  [WARN] Port {} is in use", port);
    }
    
    println!("\nDiagnostics complete");
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
    
    println!("ModelLink Auto Update\n");
    
    let current_version = cargo_crate_version!();
    println!("Current version: v{}", current_version);
    
    let release = Update::configure()
        .repo_owner("954510662-bot")
        .repo_name("ModelLink")
        .bin_name("model-link")
        .show_download_progress(true)
        .current_version(current_version)
        .build()?;
    
    println!("Checking for updates...");
    let latest_release = release.get_latest_release()?;
    
    if latest_release.version == current_version {
        println!("Already up to date!");
        return Ok(());
    }
    
    println!("New version found: v{}", latest_release.version);
    println!("Release date: {}", latest_release.date);
    println!("\nRelease notes:\n{}", latest_release.body);
    
    if check_only {
        return Ok(());
    }
    
    if !no_confirm {
        println!("\nContinue with update? (y/N)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Update cancelled");
            return Ok(());
        }
    }
    
    println!("\nStarting update...");
    let status = release.update()?;
    
    match status {
        self_update::Status::UpToDate => {
            println!("Already up to date!");
        }
        self_update::Status::Updated(v) => {
            println!("Update successful! Now running v{}", v);
        }
    }
    
    Ok(())
}
