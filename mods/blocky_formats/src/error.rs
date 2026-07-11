use std::path::PathBuf;

/// Errors returned by file and JSON helpers.
#[derive(Debug, thiserror::Error)]
pub enum BlockyError {
    #[error("failed to read {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid blocky asset: {0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, BlockyError>;
