use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub fn create_producer() -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create Kafka producer")
}

pub async fn produce(producer: FutureProducer, msg: String) {
    let record = FutureRecord::to("transaction-events")
        .payload(&msg)
        .key("txn");

    match producer.send(record, Timeout::After(Duration::from_secs(1))).await {
        Ok(delivery) => println!("Message sent: {:?}", delivery),
        Err((err, _)) => eprintln!("Failed to send message: {:?}", err),
    }
}
