use http::{HeaderValue, Method};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, CorsLayer};
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
    setup_db(&config).await?;
    let db_pool = db::setup_db_pool(&config.database).await?;
    let server_addr = config.server.address.parse()?;
    tracing::info!("Starting gRPC server on {}", server_addr);

    let cors = CorsLayer::new()
        .allow_origin(config.website.url.parse::<HeaderValue>()?)
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([
            "content-type".parse()?,
            "x-grpc-web".parse()?,
            "x-user-agent".parse()?,
        ])
        .expose_headers(["grpc-status".parse()?, "grpc-message".parse()?]);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(todo_backend::core::init_domain(db_pool.clone()).await?)
        .serve(server_addr)
        .await?;

    Ok(())
}

async fn setup_db(config: &config::Config) -> anyhow::Result<()> {
    let db_pool = db::setup_db_pool(&config.admin_db).await?;
    todo_backend::core::run_migrations(&db_pool).await?;
    return Ok(());
}
