use crate::errors::{ChunkError, ProcessorError};
use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    InvalidArguments(String),
    ThreadPool(String),
    Chunk(ChunkError),
    Processor(ProcessorError),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::InvalidArguments(msg) => write!(f, "invalid arguments: {}", msg),
            EngineError::ThreadPool(msg) => write!(f, "thread pool initialization failed: {}", msg),
            EngineError::Chunk(err) => write!(f, "chunk error: {}", err),
            EngineError::Processor(err) => write!(f, "processor error: {}", err),
        }
    }
}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::Chunk(ChunkError::Io(err))
    }
}

impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::InvalidArguments(_) | EngineError::ThreadPool(_) => None,
            EngineError::Chunk(err) => Some(err),
            EngineError::Processor(err) => Some(err),
        }
    }
}

impl From<ChunkError> for EngineError {
    fn from(err: ChunkError) -> Self {
        EngineError::Chunk(err)
    }
}

impl From<ProcessorError> for EngineError {
    fn from(err: ProcessorError) -> Self {
        EngineError::Processor(err)
    }
}
