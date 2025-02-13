use rand::Rng;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create Kafka producer");

    let mut rng = rand::thread_rng();

    for _ in 0..1_000_000 {
        let user_id: u64 = rng.gen();
        let record = FutureRecord::to("user_events")
            .key("")
            .payload(&user_id.to_string());

        producer.send(record, Duration::from_secs(0)).await.expect("Failed to send message");
    }

    println!("Produced 1M user events to Kafka!");
}
