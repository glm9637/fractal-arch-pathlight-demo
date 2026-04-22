use crate::system::get_engine;
use crate::{api::command::trigger_refresh::TriggerRefreshCommand, domain::AuthState};
use tokio::sync::watch;
use tonic::{Code, Status};

#[derive(Clone, Debug)]
pub struct AuthRetryManager {
    state_rx: watch::Receiver<AuthState>,
}

pub fn init_retry_manager() -> AuthRetryManager {
    tracing::info!("Initializing retry manager");

    AuthRetryManager {
        state_rx: get_engine().subscribe(),
    }
}

impl AuthRetryManager {
    pub async fn execute<C, F, Fut, T>(&self, client: &C, mut request_fn: F) -> Result<T, Status>
    where
        C: Clone,
        F: FnMut(C) -> Fut,
        Fut: std::future::Future<Output = Result<T, Status>>,
    {
        let mut rx = self.state_rx.clone();

        tracing::info!("starting request.");
        if rx.borrow().needs_refresh {
            tracing::info!("Token refresh already in progress. Waiting before firing API call...");
            let _ = rx.wait_for(|state| !state.needs_refresh).await;
        }

        let attempt_client = client.clone();
        match request_fn(attempt_client).await {
            Err(status)
                if status.code() == Code::Unauthenticated
                    || (status.code() == Code::Unknown
                        && status.message().contains("invalid content type")) =>
            {
                tracing::warn!("401 Unauthenticated. Triggering token refresh...");

                // POINT 2: Safely trigger the Dart refresh
                if let Err(e) = get_engine().dispatch(TriggerRefreshCommand {}).await {
                    tracing::error!("CRITICAL: Failed to dispatch refresh command: {}", e);
                    return Err(status); // Cannot refresh, return the original 401
                }

                // Wait until Dart clears the flag safely.
                // If is_err() triggers, the state channel dropped entirely.
                if rx.wait_for(|state| !state.needs_refresh).await.is_err() {
                    tracing::error!(
                        "CRITICAL: State channel closed while waiting for token refresh"
                    );
                    return Err(status);
                }

                tracing::info!("Token refreshed. Executing final retry...");
                let final_client = client.clone();
                request_fn(final_client).await
            }

            // Success, or a non-auth error (like 404 Not Found)
            other => other,
        }
    }
}
