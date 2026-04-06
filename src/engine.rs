use std::{env, fs::File, path::PathBuf, thread::available_parallelism};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;

use crate::{chunk::divide_chunks, partial_result::PartialResult, processor::Processor};

/// Orchestrates the parallel processing of the dataset.
pub struct Engine {
    path: PathBuf,
    workers: usize,
}

impl Engine {
    /// Creates a new Engine with the given path and number of workers.
    pub fn new(path: PathBuf, workers: usize) -> Self {
        Self { path, workers }
    }

    /// Creates an Engine from command line arguments.
    /// Usage: `cargo run -- <path> [workers]`
    /// If workers is not provided, uses the number of available CPUs.
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let path = PathBuf::from(&args[1]);
        let workers = args
            .get(2)
            .and_then(|w| w.parse().ok())
            .unwrap_or_else(|| available_parallelism().unwrap().get());

        ThreadPoolBuilder::new()
            .num_threads(workers)
            .build_global()
            .unwrap();

        Self { path, workers }
    }

    /// Divides the file into chunks and processes each chunk in parallel.
    /// Returns the merged PartialResult of all workers.
    pub fn run(&self) -> PartialResult {
        let mut file = File::open(&self.path).unwrap();
        let chunks = divide_chunks(&mut file, self.workers).unwrap();

        chunks
            .into_par_iter()
            .map(|chunk| Processor::new(&self.path, chunk).process_chunk())
            .reduce(PartialResult::new, |a, b| a.merge(b))
    }
}