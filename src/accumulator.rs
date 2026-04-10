/// Accumulates sum and count for computing averages incrementally.
#[derive(Debug)]
pub struct Accumulator {
    pub sum: f64,
    pub count: u64,
}

impl Accumulator {
    /// Creates a new Accumulator with the given value.
    pub fn new() -> Self {
        Self { sum: 0.0, count: 0 }
    }

    pub fn add(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }

    /// Computes the average.
    pub fn average(&self) -> f64 {
        self.sum / self.count as f64
    }

    /// Merges another Accumulator into this one. 
    pub fn merge(&mut self, other: &Accumulator) { 
        self.sum += other.sum; 
        self.count += other.count; 
    }

}