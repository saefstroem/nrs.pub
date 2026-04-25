use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Chain {0} not found")]
    NotFound(u64),

    #[error("Chain {0} already exists")]
    AlreadyExists(u64),

    #[error("Invalid RPC URL: {0}")]
    InvalidUrl(String),
}
