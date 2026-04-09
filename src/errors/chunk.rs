use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ChunkError {
    Io(io::Error),
    InvalidChunkCount(String),
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::Io(err) => write!(f, "I/O error while dividing chunks: {}", err),
            ChunkError::InvalidChunkCount(msg) => {
                write!(f, "invalid number of chunks: {}", msg)
            }
        }
    }
}

impl std::error::Error for ChunkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChunkError::Io(err) => Some(err),
            ChunkError::InvalidChunkCount(_) => None,
        }
    }
}

impl From<io::Error> for ChunkError {
    fn from(err: io::Error) -> Self {
        ChunkError::Io(err)
    }
}
