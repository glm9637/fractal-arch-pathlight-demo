use tokio::sync::watch;
use tonic::metadata::MetadataValue;
use tonic::{Request, Status, service::Interceptor};

use crate::domain::AuthState;
use crate::system::get_engine;

#[derive(Clone)]
pub struct AuthInterceptor {
    state_rx: watch::Receiver<AuthState>,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        // .borrow() is SYNCHRONOUS and lock-free!
        // It perfectly bridges your async engine to Tonic's sync trait.
        let state = self.state_rx.borrow();

        if let Some(token) = state.access_token.clone() {
            let bearer_string = format!("Bearer {}", token);

            let meta_value = MetadataValue::try_from(&bearer_string)
                .map_err(|_| Status::unauthenticated("Invalid token characters"))?;

            request.metadata_mut().insert("authorization", meta_value);
        }

        Ok(request)
    }
}

pub fn init_interceptor() -> AuthInterceptor {
    return AuthInterceptor {
        state_rx: get_engine().subscribe(),
    };
}
