use sqlx::sqlite::SqlitePoolOptions;

use crate::config::DatabaseConfig;

pub async fn setup_db_pool(database: &DatabaseConfig) -> anyhow::Result<sqlx::SqlitePool> {
    tracing::info!("Connecting to database: {}", database.url);
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database.url)
        .await?;
    Ok(db_pool)
}
