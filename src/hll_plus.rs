use std::collections::HashSet;

 // For improved accuracy with small counts
pub struct HyperLogLogPlus {
    registers: Vec<u8>,
    hash_set: HashSet<u64>,
}

impl HyperLogLogPlus {
    pub fn new(size: usize) -> Self {
        Self {
            registers: vec![0; size],
            hash_set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: u64) {
        self.hash_set.insert(value);

        let hash = value.wrapping_mul(11400714819323198549)
        let index = (hash % self.registers.len() as u64) as usize;
        let leading_zeros = hash.leading_zeros() as u8 + 1;
        self.registers[index] = self.registers[index].max(leading_zeros);
    }

    pub fn estimate(&self) -> usize {
        let raw_estimate: f64 = self.registers.iter().map(|&r| 2.0_f64.powi(-(r as i32))).sum();
        let harmonic_mean = 1.0 / raw_estimate;
        let estimate = (self.registers.len() as f64 * harmonic_mean) as usize;
        
        if self.hash_set.len() < self.registers.len() {
            return self.hash_set.len();
        }
        estimate
    }
}
