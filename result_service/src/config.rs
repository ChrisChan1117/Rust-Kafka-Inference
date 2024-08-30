// src/config.rs

use dotenv;
use std::time::Duration;

pub const MAX_RETRIES: u32 = 3;
pub const RETRY_DELAY: Duration = Duration::from_secs(2);

pub struct AppConfig {
    pub consume_topic: String,
    pub broker: String,
    pub group_id: String,
    pub database_url: String,
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let consume_topic =
            std::env::var("CONSUME_TOPIC_NAME").unwrap_or("result-topic".to_string());
        let broker = std::env::var("BROKER").unwrap_or("kafka:9092".to_string());
        let group_id = std::env::var("GROUP_ID").unwrap_or("group1".to_string());
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@db/resultdb".to_string());
        let port = std::env::var("WEBSOCKET_PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .unwrap_or(8080);

        AppConfig {
            consume_topic,
            broker,
            group_id,
            database_url,
            port,
        }
    }
}
