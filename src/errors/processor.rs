use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ProcessorError {
    Io(io::Error),
    Parse(String),
    InvalidRecord(String),
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorError::Io(err) => write!(f, "I/O error while processing chunk: {}", err),
            ProcessorError::Parse(msg) => write!(f, "parse error: {}", msg),
            ProcessorError::InvalidRecord(msg) => write!(f, "invalid record: {}", msg),
        }
    }
}

impl std::error::Error for ProcessorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessorError::Io(err) => Some(err),
            ProcessorError::Parse(_) | ProcessorError::InvalidRecord(_) => None,
        }
    }
}

impl From<io::Error> for ProcessorError {
    fn from(err: io::Error) -> Self {
        ProcessorError::Io(err)
    }
}
