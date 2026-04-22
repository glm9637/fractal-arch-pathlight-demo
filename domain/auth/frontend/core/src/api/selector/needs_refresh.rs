use state_machine::selector::{DataSink, Selectable};

use crate::{
    domain::{AuthDomain, AuthState},
    system,
};

#[derive(PartialEq, Debug, Clone)]
pub struct NeedsRefresh {
    pub needs_refresh: bool,
}

impl Selectable<AuthDomain> for NeedsRefresh {
    fn from_state(state: &AuthState) -> Self {
        return NeedsRefresh {
            needs_refresh: state.needs_refresh,
        };
    }
}

pub async fn watch_needs_refresh(sink: Box<dyn DataSink<NeedsRefresh>>) {
    system::get_engine().add_selector(sink).await;
}
