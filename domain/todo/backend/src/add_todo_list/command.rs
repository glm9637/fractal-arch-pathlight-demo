use todo_api_server::v1::{request::AddTodoEntryRequest, response::AddTodoEntryResponse};

pub async fn add_todo_entry(
    db_pool: &sqlx::SqlitePool,
    data: &AddTodoEntryRequest,
) -> anyhow::Result<AddTodoEntryResponse> {
    let result = super::query::add_todo_entry(db_pool, data).await?;
    let response = super::map::map_todo_record_to_response_entry(result);
    return Ok(response);
}
