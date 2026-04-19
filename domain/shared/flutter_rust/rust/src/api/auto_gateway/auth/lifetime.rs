// AUTO-GENERATED

pub use auth_frontend_core::api::lifetime::AuthSystemConfig;

#[flutter_rust_bridge::frb(mirror(AuthSystemConfig))]
pub struct _AuthSystemConfig { pub base_url : String , }


pub fn init_auth_system(config: AuthSystemConfig) -> anyhow::Result<()> {
    <auth_frontend_core::api::lifetime::AuthState as state_machine::lifetime::SystemInit<AuthSystemConfig>>::init_system(config)
}


pub fn dispose_auth_system() -> anyhow::Result<()> {
    <auth_frontend_core::api::lifetime::AuthSystemConfig as state_machine::lifetime::SystemDispose>::dispose_system()
}
