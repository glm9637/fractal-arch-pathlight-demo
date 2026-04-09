pub trait Reaction<S: Clone + Send + Sync> {
    fn trigger(&self, old_state: &S, new_state: &S) -> bool;
    fn execute(&self, state: &S);
}
