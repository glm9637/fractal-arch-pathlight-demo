use crate::integration::domain::auth::auth_interceptor::AuthInterceptor;

use state_machine::{
    context::EngineContext, domain::Domain, engine::FractalEngine, network::UniversalChannel,
};
use todo_api_client::{
    tonic::service::interceptor::InterceptedService,
    v1::service::todo_service_client::TodoServiceClient,
};

#[derive(Clone, Default, Debug)]
pub struct TodoState {
    pub items: Vec<String>,
    pub is_loading: bool,
}

pub type TodoEngine = FractalEngine<TodoDomain>;
pub type TodoContext = EngineContext<TodoDomain>;
pub type TodoClient = TodoServiceClient<InterceptedService<UniversalChannel, AuthInterceptor>>;

#[derive(Clone, Debug)]
pub struct TodoResources {
    pub client: TodoClient,
}
pub struct TodoDomain;

impl Domain for TodoDomain {
    type State = TodoState;
    type Resources = TodoResources;
}
