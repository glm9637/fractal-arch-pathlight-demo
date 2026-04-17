use todo_api_server::v1::request::AddTodoEntryRequest;

use super::model::TodoRecord;

pub async fn add_todo_entry(
    db_pool: &sqlx::PgPool,
    entry: &AddTodoEntryRequest,
) -> anyhow::Result<TodoRecord> {
    let id = uuid::Uuid::new_v4();
    let result = sqlx::query_as!(
        TodoRecord,
        r#"
        INSERT INTO todos
            (id, title, is_completed)
        VALUES
            ($1,$2,$3)
        RETURNING
            id as "id: _",
            title,
            is_completed
        "#,
        id,
        entry.title,
        entry.completed
    )
    .fetch_one(db_pool)
    .await?;

    return Ok(result);
}
