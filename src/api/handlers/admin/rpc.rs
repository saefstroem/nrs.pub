use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::api::{Api, ApiState, error::ApiError, result::Result};
use crate::rpc::error::RpcError;

#[derive(Deserialize)]
pub struct RpcUrlRequest {
    url: String,
}

impl Api {
    /// POST /api/v1/admin/chains/:chain_id/rpcs — add an RPC URL to a chain
    pub async fn admin_add_rpc(
        State(state): State<Arc<ApiState>>,
        headers: HeaderMap,
        Path(chain_id): Path<u64>,
        Json(body): Json<RpcUrlRequest>,
    ) -> Result<impl IntoResponse> {
        super::require_auth(&state, &headers)?;
        state
            .rpc_storage
            .write()
            .add_rpc(chain_id, body.url)
            .map_err(|e| match e {
                RpcError::NotFound(_) => ApiError::NotFound(e.to_string()),
                RpcError::InvalidUrl(_) => ApiError::BadRequest(e.to_string()),
                _ => ApiError::Internal(e.to_string()),
            })?;
        Ok(axum::http::StatusCode::CREATED)
    }

    /// DELETE /api/v1/admin/chains/:chain_id/rpcs — remove an RPC URL from a chain
    pub async fn admin_remove_rpc(
        State(state): State<Arc<ApiState>>,
        headers: HeaderMap,
        Path(chain_id): Path<u64>,
        Json(body): Json<RpcUrlRequest>,
    ) -> Result<impl IntoResponse> {
        super::require_auth(&state, &headers)?;
        state
            .rpc_storage
            .write()
            .remove_rpc(chain_id, &body.url)
            .map_err(|e| match e {
                RpcError::NotFound(_) => ApiError::NotFound(e.to_string()),
                _ => ApiError::Internal(e.to_string()),
            })?;
        Ok(axum::http::StatusCode::NO_CONTENT)
    }
}
