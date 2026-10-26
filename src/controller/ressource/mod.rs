use std::sync::Arc;

use axum::{Router, http::StatusCode, response::IntoResponse};

use crate::AppState;

pub mod catalog;
pub mod celestial_body;

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/catalog", catalog::get_routes())
        .nest("/celestial-body", celestial_body::get_routes())
}

#[derive(Clone)]
pub enum ApiError {
    NotFound,
    Internal,
    BadRequest,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {

        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Catalog resource not found").into_response(),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response(),
            ApiError::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
        }
    }
}
