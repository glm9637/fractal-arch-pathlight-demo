use state_machine::{
    context::EngineContext, domain::Domain, engine::FractalEngine, network::UniversalChannel,
};
use todo_api_client::v1::service::todo_service_client::TodoServiceClient;

#[derive(Clone, Default, Debug)]
pub struct TodoState {
    pub items: Vec<String>,
    pub is_loading: bool,
}

pub type TodoEngine = FractalEngine<TodoDomain>;
pub type TodoContext = EngineContext<TodoDomain>;

#[derive(Clone, Debug)]
pub struct TodoResources {
    pub client: TodoServiceClient<UniversalChannel>,
}
pub struct TodoDomain;

impl Domain for TodoDomain {
    type State = TodoState;
    type Resources = TodoResources;
}
