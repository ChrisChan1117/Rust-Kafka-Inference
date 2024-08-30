// src/config.rs

use dotenv;

pub struct AppConfig {
    pub broker: String,
    pub port: String,
    pub topic: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let broker = std::env::var("BROKER").unwrap_or("kafka:9092".to_string());
        let port = std::env::var("PORT").unwrap_or("3010".to_string());
        let topic = std::env::var("PRODUCE_TOPIC_NAME").unwrap_or("data-topic".to_string());

        AppConfig {
            broker,
            port,
            topic,
        }
    }
}
