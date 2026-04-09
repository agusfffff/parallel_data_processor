use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::partial_result::PartialResult;

#[derive(Debug)]
pub struct FinalResult {
    pub t1_max: ((u16, u8, u8), f64),
    pub t2_max: ((u16, i32, i32), f64),
}

pub struct Aggregator;

impl Aggregator {
    pub fn aggregate(result: PartialResult) -> FinalResult {
        let t1_max = result.t1
            .into_par_iter()
            .reduce_with(|a, b| if a.1.average() >= b.1.average() { a } else { b })
            .map(|(k, acc)| (k, acc.average()))
            .expect("T1 vacío");

        let t2_max = result.t2
            .into_par_iter()
            .reduce_with(|a, b| if a.1.average() >= b.1.average() { a } else { b })
            .map(|(k, acc)| (k, acc.average()))
            .expect("T2 vacío");

        FinalResult { t1_max, t2_max }
    }
}