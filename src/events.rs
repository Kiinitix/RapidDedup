use rand::{Rng, thread_rng};

pub fn generate_fake_user_ids(n: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen::<u64>()).collect()
}
