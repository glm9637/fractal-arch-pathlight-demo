pub trait Domain: Send + Sync + 'static {
    type State: Clone + Send + Sync + std::fmt::Debug;
    type Resources: Clone + Send + Sync;
}
