mod events;
mod hll_plus;

use hll_plus::HyperLogLogPlus;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use events::generate_fake_user_ids;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let hll = Arc::new(Mutex::new(HyperLogLogPlus::new(2048)));

    let user_events = generate_fake_user_ids(1_000_000);

    user_events.par_iter().for_each(|user_id| {
        let mut hll_lock = hll.lock().unwrap();
        hll_lock.insert(*user_id);
    });

    let estimated_count = hll.lock().unwrap().estimate();
    let elapsed_time = start_time.elapsed();

    println!("\nNumber of fake user ids generated: 1,000,000");
    println!("Estimated unique users (Parallel HLL++): {}", estimated_count);
    println!("Execution Time: {:.2?}", elapsed_time);
}