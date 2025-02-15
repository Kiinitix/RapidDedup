use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_stream::StreamExt;
mod hll_plus;
use crate::hll_plus::HyperLogLogPlus;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "deduplication_group")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["test-topic"]).expect("Failed to subscribe");

    let mut hll = HyperLogLogPlus::new(1024);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("benchmark.csv")
        .expect("Failed to open benchmark file");

    while let Some(Ok(message)) = consumer.stream().next().await {
        if let Some(payload) = message.payload() {
            let payload_str = String::from_utf8_lossy(payload);
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time error")
                .as_millis();

            let kafka_timestamp = message.timestamp().to_millis().unwrap_or(0);

            let message_hash = hash_payload(&payload_str);
            let is_duplicate = hll.estimate() > hll.get_unique_count();
            hll.insert(message_hash);

            let dedup_status = if is_duplicate { "duplicate" } else { "unique" };

            writeln!(file, "{},{},{}", timestamp, kafka_timestamp, dedup_status)
                .expect("Failed to write to benchmark file");

            println!("Processed: {}, Status: {}", payload_str, dedup_status);
        }
    }
}

fn hash_payload(payload: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(payload, &mut hasher);
    std::hash::Hasher::finish(&hasher)
}
