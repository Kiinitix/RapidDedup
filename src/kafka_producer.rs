use std::time::{SystemTime, UNIX_EPOCH};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    println!("Starting Kafka Producer...");

    for i in 0..1000 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time error")
            .as_millis();

        let message = format!("event_{}:{}", i, now);
        let key = hash_payload(&message).to_string();

        let _ = producer.send(
            FutureRecord::to("test-topic")
                .key(&key)
                .payload(&message),
            Duration::from_secs(0),
        ).await;

        println!("Produced message: {}", message);
        sleep(Duration::from_millis(10)).await;
    }
}

fn hash_payload(payload: &str) -> u64 {
    let mut hash = 5381u64;
    for byte in payload.as_bytes() {
        hash = (hash.wrapping_shl(5)).wrapping_add(hash) ^ (*byte as u64);
    }
    hash
}
