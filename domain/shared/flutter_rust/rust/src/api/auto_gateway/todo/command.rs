// AUTO-GENERATED

pub use todo_frontend_core::api::commands::load_todos::LoadTodosCommand;

#[flutter_rust_bridge::frb(mirror(LoadTodosCommand))]
pub struct _LoadTodosCommand { pub limit : usize , pub offset : usize , }


pub async fn dispatch_load_todos_command(command: LoadTodosCommand) -> anyhow::Result<()> {
    todo_frontend_core::system::get_engine().dispatch(command).await 
}

pub use todo_frontend_core::api::commands::add_todo::AddTodoCommand;

#[flutter_rust_bridge::frb(mirror(AddTodoCommand))]
pub struct _AddTodoCommand { pub text : String , }


pub async fn dispatch_add_todo_command(command: AddTodoCommand) -> anyhow::Result<()> {
    todo_frontend_core::system::get_engine().dispatch(command).await 
}
