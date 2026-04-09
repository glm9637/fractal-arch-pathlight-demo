use crate::{context::EngineContext, domain::Domain};

pub trait Command<D: Domain> {
    async fn execute(&self, ctx: &EngineContext<D>);
}
