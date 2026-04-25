use std::sync::Arc;

use axum::extract::Path;
use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;

use crate::api::error::ApiError;
use crate::api::result::Result;
use crate::api::{Api, ApiState};
use crate::rpc::ChainInfo;
use crate::stats::ChainStats;

#[derive(Serialize)]
struct GetChainResponse {
    chain_info:ChainInfo,
    stats: Option<ChainStats>,
}

impl Api {
    /// Handler for the GET /api/v1/chains endpoint.
    /// Returns a list of all supported chains.
    pub async fn get_chains(State(state): State<Arc<ApiState>>) -> Result<impl IntoResponse> {
        let chains = state
            .rpc_storage
            .read()
            .all_chains()
            .iter()
            .map(|chain| chain.chain_id)
            .collect::<Vec<u64>>();
        Ok(Json(chains))
    }

    /// Handler for a single chain
    pub async fn get_chain(
        State(state): State<Arc<ApiState>>,
        Path(chain_id): Path<u64>,
    ) -> Result<impl IntoResponse> {
        let chain = state
            .rpc_storage
            .read()
            .all_chains()
            .into_iter()
            .find(|c| c.chain_id == chain_id)
            .ok_or_else(|| ApiError::NotFound(format!("Chain with chain_id {} not found", chain_id)))?;

        let monitor = state.monitor.clone();
        let stats = monitor.get_chain_stats(chain_id).await;
        let response = GetChainResponse {
            chain_info: chain,
            stats,
        };
        Ok(Json(response))
    }
}
