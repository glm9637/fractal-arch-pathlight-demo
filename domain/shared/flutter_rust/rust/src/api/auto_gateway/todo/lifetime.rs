// AUTO-GENERATED

pub use todo_frontend_core::api::lifetime::TodoSystemConfig;

#[flutter_rust_bridge::frb(mirror(TodoSystemConfig))]
pub struct _TodoSystemConfig { pub base_url : String , }


pub fn init_todo_system(config: TodoSystemConfig) -> anyhow::Result<()> {
    <todo_frontend_core::api::lifetime::TodoState as state_machine::lifetime::SystemInit<TodoSystemConfig>>::init_system(config)
}


pub fn dispose_todo_system() -> anyhow::Result<()> {
    <todo_frontend_core::api::lifetime::TodoState as state_machine::lifetime::SystemDispose>::dispose_system()
}
