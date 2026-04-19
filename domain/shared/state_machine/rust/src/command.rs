use crate::{context::EngineContext, domain::Domain};

#[allow(async_fn_in_trait)]
pub trait Command<D: Domain> {
    async fn execute(&self, ctx: &EngineContext<D>);
}
