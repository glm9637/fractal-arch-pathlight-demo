// AUTO-GENERATED

pub use auth_frontend_core::api::command::set_tokens::SetTokensCommand;

#[flutter_rust_bridge::frb(mirror(SetTokensCommand))]
pub struct _SetTokensCommand { pub access_token : String , pub refresh_token : String , }


pub async fn dispatch_set_tokens_command(command: SetTokensCommand) -> anyhow::Result<()> {
    auth_frontend_core::system::get_engine().dispatch(command).await 
}

pub use auth_frontend_core::api::command::trigger_refresh::TriggerRefreshCommand;

#[flutter_rust_bridge::frb(mirror(TriggerRefreshCommand))]
pub struct _TriggerRefreshCommand { }


pub async fn dispatch_trigger_refresh_command(command: TriggerRefreshCommand) -> anyhow::Result<()> {
    auth_frontend_core::system::get_engine().dispatch(command).await 
}
