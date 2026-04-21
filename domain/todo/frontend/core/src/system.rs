use crate::integration::domain::auth::auth_interceptor::init_interceptor;
use crate::{
    api::lifetime::TodoSystemConfig,
    domain::{TodoClient, TodoEngine, TodoResources, TodoState},
};
use once_cell::sync::Lazy;
#[cfg(not(target_family = "wasm"))]
use state_machine::network::RUNTIME;
use state_machine::network::create_channel;
use std::sync::{Arc, RwLock};
use todo_api_client::v1::service::todo_service_client::TodoServiceClient;

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

fn init_client(config: TodoSystemConfig) -> anyhow::Result<TodoClient> {
    #[cfg(not(target_family = "wasm"))]
    let _guard = RUNTIME.enter();
    let interceptor = init_interceptor();
    let channel = create_channel(config.base_url)?;
    let client = TodoServiceClient::with_interceptor(channel, interceptor);
    return Ok(client);
}
