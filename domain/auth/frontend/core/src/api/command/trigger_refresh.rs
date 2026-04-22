use state_machine::command::Command;

use crate::domain::{AuthContext, AuthDomain};

pub struct TriggerRefreshCommand {}

impl Command<AuthDomain> for TriggerRefreshCommand {
    async fn execute(&self, ctx: &AuthContext) {
        ctx.update(|f| f.needs_refresh = true);
    }
}
