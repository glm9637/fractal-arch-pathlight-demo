use state_machine::lifetime::{SystemDispose, SystemInit};

use crate::system;

pub use crate::domain::AuthState;

pub struct AuthSystemConfig {
    pub base_url: String,
}

impl SystemInit<AuthSystemConfig> for AuthState {
    fn init_system(config: AuthSystemConfig) -> anyhow::Result<()> {
        return system::init_engine(AuthState::default(), config);
    }
}

impl SystemDispose for AuthSystemConfig {
    fn dispose_system() -> anyhow::Result<()> {
        return system::dispose_engine();
    }
}
