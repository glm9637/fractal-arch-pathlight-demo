use crate::domain::Domain;

pub struct EngineContext<D: Domain> {
    pub(crate) tx: tokio::sync::watch::Sender<D::State>,
    pub resources: D::Resources,
}

impl<D: Domain> EngineContext<D> {
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut D::State),
    {
        self.tx.send_modify(f);
    }

    pub fn get_state(&self) -> D::State {
        self.tx.borrow().clone()
    }
}
