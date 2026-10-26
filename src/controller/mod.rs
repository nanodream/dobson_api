pub mod ressource;
pub mod monitoring;

use std::sync::Arc;

use axum::{ Router };

use crate::AppState;

pub fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/v1", ressource::get_routes())
        .nest("/ops", monitoring::get_routes())
}