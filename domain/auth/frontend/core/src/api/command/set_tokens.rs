use state_machine::command::Command;

use crate::domain::{AuthContext, AuthDomain};

pub struct SetTokensCommand {
    pub access_token: String,
    pub refresh_token: String,
}

impl Command<AuthDomain> for SetTokensCommand {
    async fn execute(&self, ctx: &AuthContext) {
        ctx.update(|f| {
            f.access_token = Some(self.access_token.clone());
            f.refresh_token = Some(self.refresh_token.clone());
        });
    }
}
