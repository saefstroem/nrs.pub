use std::time::SystemTimeError;

use thiserror::Error;

use crate::auth::error::AuthError;
use crate::env::EnvError;
use crate::rpc::error::RpcError;

#[derive(Debug, Error)]
pub enum NrsError {
    #[error("Auth error: {0}")]
    Auth(#[from] AuthError),

    #[error("RPC error: {0}")]
    Rpc(#[from] RpcError),

    #[error("Json serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("System time error: {0}")]
    SystemTime(#[from] SystemTimeError),

    #[error("Std io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Env error: {0}")]
    Env(#[from] EnvError),
}
