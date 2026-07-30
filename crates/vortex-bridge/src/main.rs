use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use vortex_bridge::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Initializing Vortex Bridge in Substrate...");
    
    // Spin up the WebSocket listener
    server::run_server().await?;

    Ok(())
}
