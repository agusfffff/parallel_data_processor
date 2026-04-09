pub mod chunk;
pub mod engine;
pub mod processor;

pub use chunk::ChunkError;
pub use engine::EngineError;
pub use processor::ProcessorError;

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Chunk(ChunkError),
    Processor(ProcessorError),
    Engine(EngineError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Chunk(err) => write!(f, "chunk error: {}", err),
            Error::Processor(err) => write!(f, "processor error: {}", err),
            Error::Engine(err) => write!(f, "engine error: {}", err),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Chunk(err) => Some(err),
            Error::Processor(err) => Some(err),
            Error::Engine(err) => Some(err),
        }
    }
}

impl From<ChunkError> for Error {
    fn from(err: ChunkError) -> Self {
        Error::Chunk(err)
    }
}

impl From<ProcessorError> for Error {
    fn from(err: ProcessorError) -> Self {
        Error::Processor(err)
    }
}

impl From<EngineError> for Error {
    fn from(err: EngineError) -> Self {
        Error::Engine(err)
    }
}
