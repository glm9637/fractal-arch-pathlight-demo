use std::sync::Arc;

use tokio::sync::{RwLock, watch};

use crate::{
    command::Command,
    context::EngineContext,
    domain::Domain,
    selector::{DataSink, Selectable, TypedSelector, UntypedSelector},
};

pub struct FractalEngine<D: Domain> {
    state: watch::Sender<D::State>,
    pub resources: D::Resources,
    _rx: watch::Receiver<D::State>,
    // reactions: Vec<Box<dyn Reaction<D::State> + Send + Sync>>,
    selectors: Arc<RwLock<Vec<Box<dyn UntypedSelector<D::State> + Send + Sync>>>>,
}

impl<D: Domain> FractalEngine<D> {
    pub fn new(initial_state: D::State, resources: D::Resources) -> Self {
        let (tx, rx) = watch::channel(initial_state);
        Self {
            state: tx,
            resources,
            _rx: rx,
            selectors: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn subscribe(&self) -> watch::Receiver<D::State> {
        self.state.subscribe()
    }

    pub async fn add_selector<F>(&self, sink: Box<dyn DataSink<F>>)
    where
        F: Selectable<D> + PartialEq + Clone + Send + Sync + std::fmt::Debug + 'static,
    {
        let mut selector = TypedSelector {
            mapper: Box::new(|state| F::from_state(state)),
            last_value: None,
            sink,
        };
        let _ = selector.update(&self.state.borrow());
        self.selectors.write().await.push(Box::new(selector));
    }

    pub async fn dispatch<C>(&self, command: C) -> anyhow::Result<()>
    where
        C: Command<D> + Send + Sync,
    {
        let ctx = EngineContext {
            tx: self.state.clone(),
            resources: self.resources.clone(),
        };

        command.execute(&ctx).await;
        self.update_selectors().await;
        return Ok(());
    }

    async fn update_selectors(&self) {
        let current_snapshot = self.state.borrow().clone();

        let mut selectors = self.selectors.write().await;
        selectors.retain_mut(|s| s.update(&current_snapshot).is_ok());
    }
}
