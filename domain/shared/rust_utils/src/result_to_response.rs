use tonic::{Response, Status};

pub fn map_optional_result_to_grpc_response<T>(
    result: anyhow::Result<Option<T>>,
    id_for_error: &str,
) -> Result<Response<T>, Status> {
    return match result {
        Ok(Some(data)) => return Ok(Response::new(data)),
        Ok(None) => {
            return Err(Status::not_found(format!(
                "No resource found for id: {}",
                id_for_error
            )));
        }
        Err(e) => {
            tracing::error!("An internal error occurred: {}", e);
            Err(Status::internal("An internal error occurred."))
        }
    };
}

pub fn map_result_to_grpc_response<T>(result: anyhow::Result<T>) -> Result<Response<T>, Status> {
    match result {
        Ok(data) => return Ok(Response::new(data)),

        Err(e) => {
            tracing::error!("An internal error occurred: {}", e);
            return Err(Status::internal("An internal error occurred."));
        }
    }
}
