use tonic::transport::Server;
use tracing_subscriber::EnvFilter;
mod config;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let config = config::Config::from_env()?;
    tracing::info!("Configuration loaded successfully.");

    let db_pool = db::setup_db_pool(&config.database).await?;
    let server_addr = config.server.address.parse()?;
    tracing::info!("Starting gRPC server on {}", server_addr);

    Server::builder()
        .add_service(todo_backend::core::init_domain(db_pool.clone()).await?)
        .serve(server_addr)
        .await?;

    Ok(())
}
