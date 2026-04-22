use state_machine::network::UniversalChannel;
use todo_api_client::v1::request::{AddTodoEntryRequest, GetTodoListRequest};
use todo_api_client::v1::response::{AddTodoEntryResponse, GetTodoListResponse};
use todo_api_client::v1::service::todo_service_client::TodoServiceClient;
use tonic::service::interceptor::InterceptedService;

use crate::integration::domain::auth::AuthenticatedApiProxy;
use crate::integration::domain::auth::auth_interceptor::AuthInterceptor;
use crate::integration::domain::auth::implement_endpoints;
use crate::integration::domain::auth::retry::AuthRetryManager;
use tonic;
type InnerTodoClient = TodoServiceClient<InterceptedService<UniversalChannel, AuthInterceptor>>;

#[derive(Debug, Clone)]
pub struct AuthenticatedApi {
    inner: InnerTodoClient,
    manager: AuthRetryManager,
}

impl AuthenticatedApiProxy<InnerTodoClient> for AuthenticatedApi {
    fn retry_manager(&self) -> &AuthRetryManager {
        &self.manager
    }

    fn client(&self) -> &InnerTodoClient {
        &self.inner
    }
}

impl AuthenticatedApi {
    pub fn new(inner: InnerTodoClient, manager: AuthRetryManager) -> Self {
        Self { inner, manager }
    }

    implement_endpoints! {
        pub async fn add_todo_entry(AddTodoEntryRequest) -> AddTodoEntryResponse;
        pub async fn get_todo_list(GetTodoListRequest) -> GetTodoListResponse;
    }
}
