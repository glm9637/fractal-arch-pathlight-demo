use crate::integration::shared::retry::AuthRetryManager;

#[macro_export]
macro_rules! implement_endpoints {
    (
        $(pub async fn $method:ident($req:ty) -> $res:ty;)*
    ) => {
        $(
            pub async fn $method(
                &self,
                req: $req,
            // Removed :: prefix
            ) -> Result<$res, ::tonic::Status> {
                self.retry_manager().execute(self.client(), |mut c| {
                    let payload = req.clone();
                    async move {
                        // Removed :: prefix
                        c.$method(tonic::Request::new(payload)).await
                         .map(|r| r.into_inner())
                    }
                }).await
            }
        )*
    };
}

pub trait AuthenticatedApiProxy<C> {
    fn retry_manager(&self) -> &AuthRetryManager;
    fn client(&self) -> &C;
}
