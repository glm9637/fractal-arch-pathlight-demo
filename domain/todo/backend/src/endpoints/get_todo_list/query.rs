use rust_utils::pagination::SqlPagination;

use super::model::TodoRecord;

pub async fn get_todo_list(
    db_pool: &sqlx::PgPool,
    pagination: &SqlPagination,
) -> anyhow::Result<Vec<TodoRecord>> {
    let rows = sqlx::query_as!(
        TodoRecord,
        r#"
    SELECT 
        id,
        title, 
        is_completed 
        FROM todos
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        pagination.limit,
        pagination.offset
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}
