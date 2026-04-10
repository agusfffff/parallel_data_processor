use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ProcessorError {
    Io(io::Error),
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorError::Io(err) => write!(f, "I/O error while processing chunk: {}", err),
        }
    }
}

impl std::error::Error for ProcessorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessorError::Io(err) => Some(err),
        }
    }
}

impl From<io::Error> for ProcessorError {
    fn from(err: io::Error) -> Self {
        ProcessorError::Io(err)
    }
}
