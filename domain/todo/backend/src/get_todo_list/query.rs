use rust_utils::pagination::SqlPagination;

use crate::get_todo_list::model::TodoRecord;

pub async fn get_todo_list(
    db_pool: &sqlx::SqlitePool,
    pagination: &SqlPagination,
) -> anyhow::Result<Vec<TodoRecord>> {
    let rows = sqlx::query_as!(
        TodoRecord,
        r#"
    SELECT 
        id as "id: _",
        title, 
        is_completed 
        FROM todos
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        pagination.limit,
        pagination.offset
    )
    .fetch_all(db_pool)
    .await?;

    Ok(rows)
}
