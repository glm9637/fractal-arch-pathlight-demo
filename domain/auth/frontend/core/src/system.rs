use crate::{
    api::lifetime::AuthSystemConfig,
    domain::{AuthEngine, AuthResources, AuthState},
};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

pub static AUTH_ENGINE: Lazy<RwLock<Option<Arc<AuthEngine>>>> = Lazy::new(|| RwLock::new(None));

pub fn init_engine(initial_state: AuthState, _: AuthSystemConfig) -> anyhow::Result<()> {
    let resources = AuthResources {};
    let mut lock = AUTH_ENGINE.write().unwrap();
    *lock = Some(Arc::new(AuthEngine::new(initial_state, resources)));
    return Ok(());
}

pub fn get_engine() -> Arc<AuthEngine> {
    return AUTH_ENGINE
        .read()
        .unwrap()
        .as_ref()
        .expect("Engine not initialized")
        .clone();
}

pub fn dispose_engine() -> anyhow::Result<()> {
    let mut lock = AUTH_ENGINE.write().unwrap();
    *lock = None;
    return Ok(());
}
