pub mod controller;
pub mod domain;
pub mod infrastructure;

use std::sync::{Arc, Mutex};
use rmcp::{ServerHandler, handler::server::wrapper::Parameters, tool, tool_handler, tool_router, transport::{StreamableHttpServerConfig, StreamableHttpService, streamable_http_server::session::local::LocalSessionManager}};
use sqlite::Connection;
use tracing_subscriber;
use tracing::{info};
use controller::get_routes;
use crate::infrastructure::repository::{CatalogRepository, init_tables};

#[derive(Clone)]
pub struct AppState {
    pub catalog_repository: Arc<CatalogRepository>
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let sqlite_connection = Arc::new(
        Mutex::new(
            Connection::open("./catalog.db").unwrap()));
    init_tables(Arc::clone(&sqlite_connection));

    let app_state = Arc::new(AppState {
        catalog_repository: Arc::new(CatalogRepository::new(Arc::clone(&sqlite_connection)))
    });

    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false) // stateless for legacy versions too
        .with_json_response(true);       // plain JSON replies for simple tools

    let service = StreamableHttpService::new(
        || Ok(Calculator),           // a fresh handler per request
        LocalSessionManager::default().into(),
        config,
    );

    // `StreamableHttpService` is a Tower service — mount it on any router.
    //let router = axum::Router::new().nest_service("/mcp", service);

    let app = get_routes().with_state(app_state).nest_service("/mcp", service);
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    info!("Server started at 🎧 {}", address);
    let _ = axum::serve(listener, app).await;
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct AddParams {
    a: i32,
    b: i32,
}

#[derive(Clone)]
struct Calculator;

#[tool_router]
impl Calculator {
    #[tool(description = "Add two numbers")]
    fn add(&self, Parameters(AddParams { a, b }): Parameters<AddParams>) -> String {
        (a + b).to_string()
    }
}

#[tool_handler(name = "calculator", version = "1.0.0", instructions = "A simple calculator")]
impl ServerHandler for Calculator {}