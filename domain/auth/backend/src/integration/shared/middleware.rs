use crate::integration::shared::claims::AuthenticatedUser;
use crate::integration::shared::validation::TokenValidator;
use axum::{
    Extension,
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::{StatusCode, header};
use std::sync::Arc;
use tonic::Status;

pub async fn async_auth_middleware(
    Extension(validator): Extension<Arc<TokenValidator>>,
    mut req: Request,
    next: Next,
) -> Response {
    let is_strict_grpc = req
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| {
            v.as_bytes().starts_with(b"application/grpc")
                && !v.as_bytes().starts_with(b"application/grpc-web")
        })
        .unwrap_or(false);

    // 2. Extract the token
    let token = match req.headers().get("authorization") {
        Some(value) => match value.to_str() {
            Ok(str_val) if str_val.starts_with("Bearer ") => &str_val[7..],
            _ => return reject(is_strict_grpc, "Invalid header format"),
        },
        None => return reject(is_strict_grpc, "Missing authorization header"),
    };

    if let Some(claims) = validator.validate(token).await {
        req.extensions_mut()
            .insert(AuthenticatedUser { id: claims.sub });

        next.run(req).await
    } else {
        reject(is_strict_grpc, "Invalid or expired token")
    }
}

// A helper to format the error identically for both protocols
fn reject(is_strict_grpc: bool, message: &str) -> Response {
    if is_strict_grpc {
        // Native gRPC expects HTTP 200 OK with a grpc-status trailer/header indicating the error
        let status = Status::unauthenticated(message);

        // 2. Convert to HTTP, map the Tonic body to an empty Axum body, and return
        return status
            .into_http()
            .map(|_: Body| Body::empty())
            .into_response();
    } else {
        // gRPC-Web and REST expect standard HTTP error codes
        return (StatusCode::UNAUTHORIZED, message.to_string()).into_response();
    }
}
