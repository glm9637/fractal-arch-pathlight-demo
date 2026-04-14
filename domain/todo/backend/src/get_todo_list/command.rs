use rust_utils::pagination::SqlPagination;
use shared_models::v1::PaginationData;
use todo_api_server::v1::response::GetTodoListResponse;

pub async fn get_todo_list(
    db_pool: &sqlx::PgPool,
    pagination: &Option<PaginationData>,
) -> anyhow::Result<GetTodoListResponse> {
    let pagination = SqlPagination::from_proto(pagination, 50);
    let todo_records = crate::get_todo_list::query::get_todo_list(db_pool, &pagination).await?;
    let response = crate::get_todo_list::map::map_todo_record_to_response_entry(&todo_records);
    return Ok(response);
}
