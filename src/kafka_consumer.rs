use hll_plus::HyperLogLogPlus;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use std::sync::{Arc, Mutex};

fn main() {
    let hll = Arc::new(Mutex::new(HyperLogLogPlus::new(2048)));

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "event-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["user_events"]).expect("Subscription failed");

    for message in consumer.iter() {
        if let Some(payload) = message.payload() {
            if let Ok(user_id) = String::from_utf8(payload.to_vec()).and_then(|s| s.parse::<u64>()) {
                let mut hll_lock = hll.lock().unwrap();
                hll_lock.insert(user_id);
            }
        }
    }
}
