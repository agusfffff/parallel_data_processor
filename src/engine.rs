use std::io::BufReader;
use std::{env, fs::File, path::PathBuf, thread::available_parallelism};

use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::aggregator::{Aggregator, FinalResult};
use crate::errors::ProcessorError;
use crate::processor::process_chunk;
use crate::{
    chunk::divide_chunks,
    errors::{EngineError},
    partial_result::PartialResult,
};

/// Orchestrates the parallel processing of the dataset.
pub struct Engine {
    path: PathBuf,
    workers: usize,
}

impl Engine {

    pub fn from_env() -> Result<Self, EngineError> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        let path = env::var("DATASET_PATH")
            .map(PathBuf::from)
            .map_err(|_| EngineError::InvalidArguments("DATASET_PATH not set".into()))?;

        let workers = env::var("WORKERS")
            .ok()
            .and_then(|w| w.parse().ok())
            .unwrap_or_else(|| available_parallelism().map(|n| n.get()).unwrap_or(1));

        ThreadPoolBuilder::new()
            .num_threads(workers)
            .build_global()
            .map_err(|e| EngineError::ThreadPool(e.to_string()))?;

        Ok(Self { path, workers })
    }

    pub fn run(&self) -> Result<FinalResult, EngineError> {
        let mut file = File::open(&self.path).map_err(EngineError::from)?;
        let chunks = divide_chunks(&mut file, self.workers)?;
        let path = self.path.clone();
        let results = chunks
            .into_par_iter()
            .map(|chunk| { 
                let file = File::open(&path).map_err(ProcessorError::from)?;
                let mut reader = BufReader::new(file);            
                process_chunk(&mut reader, chunk)
            })
            .try_reduce(PartialResult::new, |a, b| Ok(a.merge(b)))
            .map_err(EngineError::from)?; 
        
        Ok(Aggregator::aggregate(results))
    }
}
