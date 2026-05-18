use clap::Parser;
use model_link::{handle_cli, init_start_time};

#[tokio::main]
async fn main() {
    if let Err(e) = color_eyre::install() {
        eprintln!("Failed to install color_eyre: {}", e);
    }
    
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    
    init_start_time();
    
    let cli = model_link::Cli::parse();
    if let Err(e) = handle_cli(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
