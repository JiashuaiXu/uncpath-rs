use thiserror::Error;

#[derive(Error, Debug)]
pub enum UncPathError {
    #[error("Invalid UNC path format: {0}")]
    InvalidFormat(String),

    #[error("No mapping found for host/share: {0}/{1}")]
    MappingNotFound(String, String),

    #[error("Invalid mapping configuration: {0}")]
    InvalidMapping(String),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, UncPathError>;
