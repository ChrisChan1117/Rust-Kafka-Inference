// src/config.rs

use dotenv;
use std::time::Duration;

pub const MAX_RETRIES: u32 = 3;
pub const RETRY_DELAY: Duration = Duration::from_secs(2);

// Struct to hold service configuration  
pub struct AppConfig {
    pub produce_topic_name: String,
    pub consume_topic_name: String,
    pub broker_name: String,
    pub group_id: String,
    pub api_service_url: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let produce_topic_name =
            std::env::var("PRODUCE_TOPIC_NAME").unwrap_or("result-topic".to_string());
        let consume_topic_name =
            std::env::var("CONSUME_TOPIC_NAME").unwrap_or("data-topic".to_string());
        let broker_name = std::env::var("KAFKA_BROKER_NAME").unwrap_or("kafka:9092".to_string());
        let group_id = std::env::var("GROUP_ID").unwrap_or("group1".to_string());
        let api_service_url = std::env::var("LOAD_BALANCER_URL")
            .unwrap_or("http://load-balancer".to_string())
            + "/chat";

        AppConfig {
            produce_topic_name,
            consume_topic_name,
            broker_name,
            group_id,
            api_service_url,
        }
    }
}
