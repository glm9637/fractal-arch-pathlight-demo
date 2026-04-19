use state_machine::{context::EngineContext, domain::Domain, engine::FractalEngine};

#[derive(Clone, Default, Debug)]
pub struct AuthState {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AuthResources {}

pub struct AuthDomain;

impl Domain for AuthDomain {
    type State = AuthState;
    type Resources = AuthResources;
}

pub type AuthEngine = FractalEngine<AuthDomain>;
pub type AuthContext = EngineContext<AuthDomain>;
