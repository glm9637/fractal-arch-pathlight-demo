pub use prost;
pub use tonic;

pub mod v1 {
    pub mod service {
        include!("pathlight/todo/api/v1/service/pathlight.todo.api.v1.service.rs");
    }

    pub mod request {
        include!("pathlight/todo/api/v1/request/pathlight.todo.api.v1.request.rs");
    }

    pub mod response {
        include!("pathlight/todo/api/v1/response/pathlight.todo.api.v1.response.rs");
    }
}
