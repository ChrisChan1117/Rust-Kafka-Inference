use axum::{extract::Extension, routing::post, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::EnvFilter;

pub mod config;
pub mod handlers;
pub mod kafka_producer;

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    init_tracing();

    // Initialize configuration
    let config = Arc::new(RwLock::new(config::AppConfig::new()));

    // Create Kafka producer and wrap in shared state
    let shared_producer = Arc::new(RwLock::new(kafka_producer::create_producer(
        config.clone().read().await.broker.clone(),
    )));

    // Set up Axum router
    let app = Router::new()
        .route("/api/subscribe", post(handlers::text_handler))
        .layer(Extension(shared_producer))
        .layer(Extension(config.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // Run server
    run_server(app, config.clone().read().await.port.clone()).await;
}

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

async fn run_server(app: Router, port: String) {
    // Bind to a specified port and start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
