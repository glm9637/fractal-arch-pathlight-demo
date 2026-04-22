use state_machine::command::Command;
use todo_api_client::v1::request::GetTodoListRequest;

use crate::domain::{TodoClient, TodoContext, TodoDomain};

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
    async fn fetch_todos_from_network(client: TodoClient) -> anyhow::Result<Vec<String>> {
        let request = GetTodoListRequest { pagination: None };

        let response = client
            .get_todo_list(request)
            .await
            .map_err(|e| anyhow::anyhow!("gRPC Error: {}", e))?;

        let items = response.items.into_iter().map(|item| item.title).collect();
        println!("Fetched todos: {:?}", items);
        return Ok(items);
    }
}
