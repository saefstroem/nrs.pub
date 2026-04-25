use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::api::{Api, ApiState, error::ApiError, result::Result};
use crate::rpc::{RpcChain, error::RpcError};

#[derive(Deserialize)]
pub struct AddChainRequest {
    chain_id: u64,
    name: String,
    #[serde(default)]
    rpcs: Vec<String>,
}

impl Api {
    /// GET /api/v1/admin/chains — list all chains with their RPC URLs
    pub async fn admin_get_chains(
        State(state): State<Arc<ApiState>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse> {
        // Require authentication
        super::require_auth(&state, &headers)?;
        // Get all chains from storage 
        let chains: Vec<RpcChain> = state.rpc_storage.read().all_chains_owned();
        Ok(Json(chains))
    }

    /// POST /api/v1/admin/chains — add a new chain
    pub async fn admin_add_chain(
        State(state): State<Arc<ApiState>>,
        headers: HeaderMap,
        Json(body): Json<AddChainRequest>,
    ) -> Result<impl IntoResponse> {
        super::require_auth(&state, &headers)?;
        state
            .rpc_storage
            .write()
            .add_chain(body.chain_id, body.name, body.rpcs)
            .map_err(|e| match e {
                RpcError::AlreadyExists(_) => ApiError::BadRequest(e.to_string()),
                _ => ApiError::Internal(e.to_string()),
            })?;
        Ok(axum::http::StatusCode::CREATED)
    }

    /// DELETE /api/v1/admin/chains/:chain_id — remove a chain entirely
    pub async fn admin_remove_chain(
        State(state): State<Arc<ApiState>>,
        headers: HeaderMap,
        Path(chain_id): Path<u64>,
    ) -> Result<impl IntoResponse> {
        super::require_auth(&state, &headers)?;
        state
            .rpc_storage
            .write()
            .remove_chain(chain_id)
            .map_err(|e| ApiError::Internal(e.to_string()))?;
        Ok(axum::http::StatusCode::NO_CONTENT)
    }
}
