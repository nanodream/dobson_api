use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::AppState;

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(test))
}

pub async fn test() -> String {
    return String::from("Hello");
}