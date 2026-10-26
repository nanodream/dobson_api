use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::AppState;

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_celestial_bodies))
}

pub async fn list_celestial_bodies() -> String {
    return String::from("list_celestial_bodies");
}