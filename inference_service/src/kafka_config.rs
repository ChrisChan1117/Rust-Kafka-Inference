use rdkafka::config::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;

/// Creates and configures a Kafka consumer  
pub fn create_consumer(broker: &String, group_id: &String) -> StreamConsumer {
    ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", broker)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed")
}

/// Creates and configures a Kafka producer  
pub fn create_producer(broker: &String) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failed")
}
