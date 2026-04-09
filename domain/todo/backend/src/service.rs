use crate::add_todo_list::command::add_todo_entry;
use crate::get_todo_list;
use rust_utils::result_to_response;
use todo_api_server::v1::request::{AddTodoEntryRequest, GetTodoListRequest};
use todo_api_server::v1::response::{AddTodoEntryResponse, GetTodoListResponse};
use todo_api_server::v1::service::todo_service_server::TodoService;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ToDoBackendServiceImpl {
    pub sqlite_pool: sqlx::SqlitePool,
}

#[tonic::async_trait]
impl TodoService for ToDoBackendServiceImpl {
    async fn get_todo_list(
        &self,
        request: Request<GetTodoListRequest>,
    ) -> std::result::Result<Response<GetTodoListResponse>, Status> {
        let pagination = &request.get_ref().pagination;
        let response = get_todo_list::command::get_todo_list(&self.sqlite_pool, pagination).await;
        return result_to_response::map_result_to_grpc_response(response);
    }

    async fn add_todo_entry(
        &self,
        request: Request<AddTodoEntryRequest>,
    ) -> std::result::Result<Response<AddTodoEntryResponse>, Status> {
        let entry = request.get_ref();
        let response = add_todo_entry(&self.sqlite_pool, entry).await;
        return result_to_response::map_result_to_grpc_response(response);
    }
}
