use crate::{
    api::lifetime::TodoSystemConfig,
    domain::{TodoEngine, TodoResources, TodoState},
};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use todo_api_client::{
    tonic::{self, transport::Channel},
    v1::service::todo_service_client::TodoServiceClient,
};
use tokio::runtime::Runtime;

pub static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to create Tokio runtime for Pathlight"));

pub static TODO_ENGINE: Lazy<RwLock<Option<Arc<TodoEngine>>>> = Lazy::new(|| RwLock::new(None));

pub fn init_engine(initial_state: TodoState, config: TodoSystemConfig) -> anyhow::Result<()> {
    let resources = TodoResources {
        client: init_client(config)?,
    };
    let mut lock = TODO_ENGINE.write().unwrap();
    *lock = Some(Arc::new(TodoEngine::new(initial_state, resources)));

    return Ok(());
}

pub fn get_engine() -> Arc<TodoEngine> {
    return TODO_ENGINE
        .read()
        .unwrap()
        .as_ref()
        .expect("Engine not initialized")
        .clone();
}

pub fn dispose_engine() -> anyhow::Result<()> {
    let mut lock = TODO_ENGINE.write().unwrap();
    *lock = None;
    return Ok(());
}

fn init_client(config: TodoSystemConfig) -> anyhow::Result<(TodoServiceClient<Channel>)> {
    let _guard = RUNTIME.enter();

    let endpoint = tonic::transport::Endpoint::from_shared(config.base_url)?;
    let channel = endpoint.connect_lazy();
    let client = TodoServiceClient::new(channel);
    return Ok(client);
}
