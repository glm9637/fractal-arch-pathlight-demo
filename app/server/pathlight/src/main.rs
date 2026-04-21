use auth_backend::integration::shared::middleware::async_auth_middleware;
use auth_backend::integration::shared::validation::TokenValidator;
use axum::{Extension, Router, middleware};
use http::{HeaderValue, Method};
use std::sync::Arc;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
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
    let server_addr = config.server.address;

    let cors = CorsLayer::new()
        .allow_origin(config.website.url.parse::<HeaderValue>()?)
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([
            "content-type".parse()?,
            "x-grpc-web".parse()?,
            "x-user-agent".parse()?,
            "authorization".parse()?,
        ])
        .expose_headers(["grpc-status".parse()?, "grpc-message".parse()?]);

    let validator = Arc::new(TokenValidator::new(
        config.zitadel.issuer_url,
        config.zitadel.client_id,
    ));

    let todo_service = todo_backend::core::init_domain(db_pool.clone()).await?;
    let grpc_router = tonic::service::Routes::new(todo_service).into_axum_router();

    let app = Router::new()
        .merge(grpc_router)
        .layer(GrpcWebLayer::new())
        .layer(middleware::from_fn(async_auth_middleware))
        .layer(Extension(validator))
        .layer(cors);

    tracing::info!("Starting Axum gRPC server on {}", server_addr);

    let listener = tokio::net::TcpListener::bind(server_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn setup_db(config: &config::Config) -> anyhow::Result<()> {
    let db_pool = db::setup_db_pool(&config.admin_db).await?;
    todo_backend::core::run_migrations(&db_pool).await?;
    return Ok(());
}
