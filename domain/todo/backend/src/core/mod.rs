use sqlx::sqlite::SqlitePool;
use todo_api_server::v1::service::todo_service_server::TodoServiceServer;

use crate::service::ToDoBackendServiceImpl;

async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    tracing::info!("Running Todo domain database migrations...");

    sqlx::migrate!("../db").run(pool).await?;

    tracing::info!("Todo domain ready.");
    Ok(())
}

pub async fn init_domain(
    pool: SqlitePool,
) -> anyhow::Result<TodoServiceServer<ToDoBackendServiceImpl>> {
    run_migrations(&pool).await?;
    let service = ToDoBackendServiceImpl { sqlite_pool: pool };
    Ok(TodoServiceServer::new(service))
}
