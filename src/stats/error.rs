use thiserror::Error;



#[derive(Debug,Error)]
pub enum StatsError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    
    #[error("Invalid block number hex: {0}")]
    InvalidHex(String),

    #[error("Could not communicate with RPC after multiple attempts")]
    RpcUnreachable,
}