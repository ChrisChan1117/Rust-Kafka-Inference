mod config;
mod inference;
mod kafka_config;

use inference::{inference_process, InferenceInputData, InferenceOutputData};
use kafka_config::{create_consumer, create_producer};
use std::collections::HashMap;
use std::str::*;
use std::time::Duration;

use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::producer::{FutureProducer, FutureRecord};

use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::config::{AppConfig, MAX_RETRIES, RETRY_DELAY};
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

/// Commit Kafka message offset  
fn commit_offset(consumer: &StreamConsumer, msg: &BorrowedMessage) {
    match consumer.commit_message(&msg, CommitMode::Sync) {
        Ok(_) => info!("Committed offset {}", msg.offset()),
        Err(e) => error!("Failed to commit offset: {}", e),
    };
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();

    // Load environment variables
    let config = AppConfig::new();

    let consumer: StreamConsumer = create_consumer(&config.broker_name, &config.group_id);
    let producer: FutureProducer = create_producer(&config.broker_name);

    // Subscribe to topic
    consumer
        .subscribe(&[config.consume_topic_name.as_str()])
        .expect("Can't subscribe to topic");

    // Initialize retry tracker  for consuming messages
    let mut retry_tracker: HashMap<String, u32> = HashMap::new();

    // Process messages in loop
    loop {
        match consumer.recv().await {
            Err(e) => error!("Kafka Info: {}", e),
            Ok(msg) => {
                if let Some(payload) = msg.payload().and_then(|p| from_utf8(p).ok()) {
                    match serde_json::from_str::<InferenceInputData>(payload) {
                        Ok(input_data) => {
                            // Run inference process
                            let output_data: InferenceOutputData =
                                inference_process(&input_data, &config.api_service_url).await;
                            let retry_count = *retry_tracker.get(&input_data.text).unwrap_or(&0);
                            match serde_json::to_string(&output_data) {
                                Ok(output_json) => {
                                    // Retry sending message up to MAX_RETRIES
                                    let mut retries = 0;
                                    loop {
                                        match producer
                                            .send(
                                                FutureRecord::to(
                                                    config.produce_topic_name.as_str(),
                                                )
                                                .payload(&output_json)
                                                .key(""),
                                                Duration::from_secs(0),
                                            )
                                            .await
                                        {
                                            Ok(_) => {
                                                commit_offset(&consumer, &msg);
                                                info!(
                                                    "Inference Output Data Sent: {:?}",
                                                    output_data
                                                );
                                                break;
                                            }
                                            Err(e) => {
                                                if retries < MAX_RETRIES {
                                                    retries += 1;
                                                    error!("Failed to send message, retrying {}/{}: {:?}", retries, MAX_RETRIES, e);
                                                    tokio::time::sleep(RETRY_DELAY).await;
                                                } else {
                                                    error!("Failed to send message after {} retries: {:?}", MAX_RETRIES, e);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => error!("Serialization error: {}", e),
                            }
                            
                            // Handle failure to process message and update retry tracker
                            if retry_count >= MAX_RETRIES {
                                info!("Failed to process message: {}", input_data.text);
                                retry_tracker.remove(&input_data.text);
                                commit_offset(&consumer, &msg);
                            } else {
                                retry_tracker.insert(input_data.text.clone(), retry_count + 1);
                            }
                        }
                        Err(e) => {
                            error!("Error parsing input: {}", e);
                            commit_offset(&consumer, &msg);
                        }
                    }
                } else {
                    error!("Message payload is not a valid UTF-8 string");
                    commit_offset(&consumer, &msg);
                }
            }
        }
    }
}
