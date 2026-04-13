use state_machine::{command::Command, network::UniversalChannel};
use todo_api_client::{
    tonic,
    v1::{request::AddTodoEntryRequest, service::todo_service_client::TodoServiceClient},
};

use crate::domain::{TodoContext, TodoDomain};

pub struct AddTodoCommand {
    pub text: String,
}

impl Command<TodoDomain> for AddTodoCommand {
    async fn execute(&self, ctx: &TodoContext) {
        ctx.update(|s| s.is_loading = true);
        let network_result = self.persist_entry(ctx.resources.client.clone()).await;
        match network_result {
            Ok(data) => {
                ctx.update(|s| {
                    let mut result = s.items.clone();
                    result.push(data);
                    s.items = result;
                    s.is_loading = false;
                });
            }
            Err(_) => {
                ctx.update(|s| s.is_loading = false);
            }
        }
    }
}

impl AddTodoCommand {
    async fn persist_entry(
        &self,
        mut client: TodoServiceClient<UniversalChannel>,
    ) -> anyhow::Result<String> {
        let request = tonic::Request::new(AddTodoEntryRequest {
            title: self.text.clone(),
            completed: false,
        });

        let response = client
            .add_todo_entry(request)
            .await
            .map_err(|e| anyhow::anyhow!("gRPC Error: {}", e))?;

        let entry = response.into_inner();

        return Ok(entry.title);
    }
}
