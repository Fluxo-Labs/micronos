use alloc::string::{String, ToString};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Data error: {0}")]
    Data(String),

    #[error("Not initialized")]
    NotInitialized,

    #[error("Already initialized")]
    AlreadyInitialized,

    #[error("Invalid operation for current state")]
    InvalidState,

    #[error("Timeout")]
    Timeout,

    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    pub fn io(msg: &str) -> Self {
        Error::Io(msg.to_string())
    }

    pub fn memory(msg: &str) -> Self {
        Error::Memory(msg.to_string())
    }

    pub fn network(msg: &str) -> Self {
        Error::Network(msg.to_string())
    }

    pub fn not_found(msg: &str) -> Self {
        Error::NotFound(msg.to_string())
    }

    pub fn state(msg: &str) -> Self {
        Error::State(msg.to_string())
    }

    pub fn data(msg: &str) -> Self {
        Error::Data(msg.to_string())
    }
}
