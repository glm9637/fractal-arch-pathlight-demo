use state_machine::{command::Command, network::UniversalChannel};
use todo_api_client::{
    tonic,
    v1::{request::GetTodoListRequest, service::todo_service_client::TodoServiceClient},
};

use crate::domain::{TodoContext, TodoDomain};

pub struct LoadTodosCommand {
    pub limit: usize,
    pub offset: usize,
}

impl Command<TodoDomain> for LoadTodosCommand {
    async fn execute(&self, ctx: &TodoContext) {
        ctx.update(|s| s.is_loading = true);

        let result = Self::fetch_todos_from_network(ctx.resources.client.clone()).await;

        match result {
            Ok(todos) => {
                ctx.update(|s| {
                    s.items = todos;
                    s.is_loading = false;
                });
            }
            Err(_) => {
                ctx.update(|s| {
                    s.is_loading = false;
                });
            }
        }
    }
}

impl LoadTodosCommand {
    async fn fetch_todos_from_network(
        mut client: TodoServiceClient<UniversalChannel>,
    ) -> anyhow::Result<Vec<String>> {
        let request = tonic::Request::new(GetTodoListRequest { pagination: None });

        let response = client
            .get_todo_list(request)
            .await
            .map_err(|e| anyhow::anyhow!("gRPC Error: {}", e))?;

        let items = response
            .into_inner()
            .items
            .into_iter()
            .map(|item| item.title)
            .collect();
        println!("Fetched todos: {:?}", items);
        return Ok(items);
    }
}
