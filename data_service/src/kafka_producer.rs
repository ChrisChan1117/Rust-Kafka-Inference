use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Creates and configures a Kafka producer  
pub fn create_producer(broker: String) -> FutureProducer {
    info!("{:?}", broker);
    ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failed")
}

/// Sends data to Kafka topic with retries  
pub async fn produce_data(
    text: String,
    producer: Arc<RwLock<FutureProducer>>,
    topic: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let retries = 3;
    let producer = producer.read().await;

    info!("Sending text to topic: {}", topic);

    for _ in 0..retries {
        let send_result = producer
            .send(
                FutureRecord::to(&topic).payload(&text).key(""),
                tokio::time::Duration::from_secs(0),
            )
            .await;

        match send_result {
            Ok(delivery) => {
                info!("Sent: {:?}, {}", delivery, text);
                return Ok(());
            }
            Err((KafkaError::MessageProduction(some_error), _)) => {
                error!("Kafka error: {:?}", some_error);
                continue;
            }
            Err((e, _)) => {
                error!("Error sending message: {:?}", e);
                continue;
            }
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to send message after retries",
    )))
}
