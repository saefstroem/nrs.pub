mod chain;
mod rpc;

use axum::http::HeaderMap;
use tokio::time::Instant;

use crate::api::{ApiState, error::ApiError, result::Result};

fn require_auth(state: &ApiState, headers: &HeaderMap) -> Result<()> {
    // Read the bearer token from the Authorization header
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".into()))?;

    // Read current session token and expiration from state
    let session = state.session.read();
    match &*session {
        // Ensure token is matching and not expired
        Some((stored, expires_at)) if stored == token => {
            if Instant::now() > *expires_at {
                return Err(ApiError::Unauthorized("Session expired".into()));
            }
            Ok(())
        }
        Some(_) => Err(ApiError::Unauthorized("Invalid token".into())),
        None => Err(ApiError::Unauthorized("Not logged in".into())),
    }
}
