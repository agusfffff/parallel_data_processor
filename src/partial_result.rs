use std::collections::HashMap;

use crate::accumulator::Accumulator;

/// Holds partial aggregation results from a single chunk.
#[derive(Debug)]
pub struct PartialResult {
    /// T1: average NO2 per (year, month, day)
    pub t1: HashMap<(u16, u8, u8), Accumulator>,
    /// T2: average NO2 per (year, grid_lat, grid_lon)
    pub t2: HashMap<(u16, i32, i32), Accumulator>,
}

impl PartialResult {
    /// Creates an empty PartialResult.
    pub fn new() -> Self {
        Self {
            t1: HashMap::new(),
            t2: HashMap::new(),
        }
    }

    /// Merges another PartialResult into this one.
    pub fn merge(mut self, other: PartialResult) -> Self {
        for (key, acc) in other.t1 {
            self.t1.entry(key).or_insert(Accumulator::new()).merge(&acc);
        }
        for (key, acc) in other.t2 {
            self.t2.entry(key).or_insert(Accumulator::new()).merge(&acc);
        }
        self
    }
}