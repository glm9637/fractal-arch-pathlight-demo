use sqlx::postgres::PgPool;
use todo_api_server::v1::service::todo_service_server::TodoServiceServer;

use crate::service::ToDoBackendServiceImpl;

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Running Todo domain database migrations...");
    sqlx::migrate!("../db").run(pool).await?;
    tracing::info!("Todo domain ready.");
    Ok(())
}

pub async fn init_domain(
    pool: PgPool,
) -> anyhow::Result<TodoServiceServer<ToDoBackendServiceImpl>> {
    let service = ToDoBackendServiceImpl { pg_pool: pool };
    Ok(TodoServiceServer::new(service))
}
