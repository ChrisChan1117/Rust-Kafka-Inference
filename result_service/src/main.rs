mod config;
mod db_client;
mod kafka_consumer;
mod websocket;

use kafka_consumer::consume_and_log;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
use warp::Filter;
use websocket::ws_route;

use crate::config::AppConfig;
use crate::db_client::DatabaseClient;
use crate::websocket::WebsocketConnection;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    init_tracing();

    // Initialize for configuration
    let config = AppConfig::new();

    // Initialize database client
    let db_client = DatabaseClient::new(config.database_url.as_str())
        .await
        .unwrap();

    // Initialize WebSocket clients
    let clients: Arc<Mutex<WebsocketConnection>> = Arc::new(Mutex::new(WebsocketConnection::new()));
    
    // Set up Warp WebSocket route
    let ws_filter = ws_route(clients.clone());
    let routes = warp::path::end()
        .map(|| warp::reply::html("WebSocket server is running"))
        .or(ws_filter);

    // Concurrently run Kafka consumer and Warp server
    tokio::select! {
        _ = consume_and_log(&config.broker, &config.group_id, &config.consume_topic, clients.clone(), &db_client) => {},
        _ = warp::serve(routes).run(([0, 0, 0, 0], config.port)) => {},
    };
}

/// Initialize tracing for logging  
fn init_tracing() {
    let filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());
    tracing_subscriber::fmt::Subscriber::builder()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(filter)
        .init();
}
