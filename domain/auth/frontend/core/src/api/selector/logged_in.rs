use state_machine::selector::{DataSink, Selectable};

use crate::{
    domain::{AuthDomain, AuthState},
    system,
};

#[derive(PartialEq, Debug, Clone)]
pub struct LoggedIn {
    pub logged_in: bool,
}

impl Selectable<AuthDomain> for LoggedIn {
    fn from_state(state: &AuthState) -> Self {
        print!("{}", state.access_token.is_some());
        return LoggedIn {
            logged_in: state.access_token.is_some(),
        };
    }
}

pub async fn watch_todo_list(sink: Box<dyn DataSink<LoggedIn>>) {
    system::get_engine().add_selector(sink).await;
}
