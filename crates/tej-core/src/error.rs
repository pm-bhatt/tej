use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpeedTestError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Network timeout after {0}ms")]
    Timeout(u64),

    #[error("Invalid server response: {0}")]
    InvalidResponse(String),

    #[error("Test cancelled")]
    Cancelled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SpeedTestError>;
