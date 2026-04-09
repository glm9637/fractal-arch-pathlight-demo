use state_machine::selector::{DataSink, Selectable};

use crate::{
    domain::{TodoDomain, TodoState},
    system,
};

#[derive(PartialEq, Debug, Clone)]
pub struct TodoList {
    pub items: Vec<String>,
}

impl Selectable<TodoDomain> for TodoList {
    fn from_state(state: &TodoState) -> Self {
        TodoList {
            items: state.items.clone().into_iter().collect(),
        }
    }
}

pub async fn watch_todo_list(sink: Box<dyn DataSink<TodoList>>) {
    system::get_engine().add_selector(sink).await;
}
