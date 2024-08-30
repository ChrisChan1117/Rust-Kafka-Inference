extern crate rdkafka;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_postgres;

use std::sync::{Arc, Mutex};

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message};
use tracing::{error, info};

use crate::db_client::{DatabaseClient, ResultOutputData};
use crate::websocket::WebsocketConnection;
/// Consumes messages from Kafka, processes them, logs them to a database, and broadcasts over WebSocket.  
pub async fn consume_and_log(
    kafka_servers: &str,
    group_id: &str,
    topic_name: &str,
    clients: Arc<Mutex<WebsocketConnection>>,
    db_client: &DatabaseClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create and configure Kafka consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", kafka_servers)
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .map_err(|e| format!("Consumer creation failed: {}", e))?;

    // Subscribe to topic
    consumer
        .subscribe(&[topic_name])
        .map_err(|e| format!("Can't subscribe to topic: {}", e))?;

    // Process messages from Kafka
    loop {
        match consumer.recv().await {
            Err(e) => error!("Kafka Info: {}", e),
            Ok(msg) => {
                if let Some(payload_str) = msg.payload().and_then(|p| std::str::from_utf8(p).ok()) {
                    match serde_json::from_str::<ResultOutputData>(payload_str) {
                        Ok(output_data) => {
                            info!("Received: {:?}", output_data);

                            // Log output data to database
                            match db_client.log_output(&output_data).await {
                                Ok(_) => info!("Log saved to database"),
                                Err(e) => error!("Database insert error: {}", e),
                            }

                            // Broadcast message over WebSocket
                            clients.lock().unwrap().broadcast_message(&output_data);

                            // Commit offset
                            commit_offset(&consumer, &msg);
                        }
                        Err(e) => error!("Error parsing output data: {}", e),
                    }
                } else {
                    error!("Received empty payload");
                }
            }
        }
    }
}


/// Commits Kafka message offset  
fn commit_offset(consumer: &StreamConsumer, msg: &BorrowedMessage) {
    match consumer.commit_message(&msg, CommitMode::Sync) {
        Ok(_) => info!("Committed offset {}", msg.offset()),
        Err(e) => error!("Failed to commit offset: {}", e),
    };
}