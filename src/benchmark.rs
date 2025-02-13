mod events;
mod hll_plus;

use hll_plus::HyperLogLogPlus;
use std::collections::HashSet;
use std::time::Instant;
use events::generate_fake_user_ids;

fn benchmark_hashset(user_events: &[u64]) {
    let mut exact_count = HashSet::new();
    let start = Instant::now();

    for &user_id in user_events {
        exact_count.insert(user_id);
    }

    println!(
        "HashSet - Unique Users: {} | Time: {:.2?}",
        exact_count.len(),
        start.elapsed()
    );
}

fn benchmark_hyperloglog(user_events: &[u64]) {
    let mut hll = HyperLogLogPlus::new(2048);
    let start = Instant::now();

    for &user_id in user_events {
        hll.insert(user_id);
    }

    println!(
        "HyperLogLog++ - Estimated Unique Users: {} | Time: {:.2?}",
        hll.estimate(),
        start.elapsed()
    );
}

fn main() {
    let user_events = generate_fake_user_ids(1_000_000);

    println!("Benchmarking Unique User Counting Methods...");
    benchmark_hashset(&user_events);
    benchmark_hyperloglog(&user_events);
}
