use state_machine::lifetime::{SystemDispose, SystemInit};

use crate::system;

pub use crate::domain::TodoState;

pub struct TodoSystemConfig {
    pub base_url: String,
}

impl SystemInit<TodoSystemConfig> for TodoState {
    fn init_system(config: TodoSystemConfig) -> anyhow::Result<()> {
        return system::init_engine(TodoState::default(), config);
    }
}

impl SystemDispose for TodoState {
    fn dispose_system() -> anyhow::Result<()> {
        return system::dispose_engine();
    }
}
