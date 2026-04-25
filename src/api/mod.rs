mod error;
pub mod handlers;
mod result;
use std::sync::Arc;

use crate::{api::error::ApiError, auth::PasswordStore, rpc::RpcStorage, stats::Monitor};
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
};
use axum::{extract::Path, extract::Request, http::Method};
use parking_lot::RwLock;
use result::Result;
use tokio::time::Instant;
use tower_http::services::{ServeDir, ServeFile};

/// Contains the state of the API, which is also shared
/// with the rest of the system.
pub struct ApiState {
    pub rpc_storage: Arc<parking_lot::RwLock<RpcStorage>>,
    pub monitor: Arc<Monitor>,
    pub pw_store: Arc<PasswordStore>,
    /// Active session: (token, expiry instant). Replaced on each successful login.
    /// There can only be one active session at a time, and it's stored in-memory for simplicity.
    pub session: RwLock<Option<(String, Instant)>>,
}

impl ApiState {
    pub fn new(
        rpc_storage: Arc<parking_lot::RwLock<RpcStorage>>,
        monitor: Arc<Monitor>,
        pw_store: Arc<PasswordStore>,
    ) -> Self {
        Self {
            rpc_storage,
            monitor,
            pw_store,
            session: RwLock::new(None),
        }
    }
}

/// Represents the main API control struct.
/// Wraps around an Axum Router and provides handler functions for each API endpoint.
pub struct Api {
    router: Router,
}
impl Api {
    async fn rpc_or_spa(
        method: Method,
        Path(chain_id): Path<String>,
        state: State<Arc<ApiState>>,
    ) -> Result<Response> {
        if method == Method::POST {
            if let Ok(id) = chain_id.parse::<u64>() {
                return Api::rpc_proxy(state, Path(id)).await;
            }
        }
        let body = tokio::fs::read_to_string("dist/200.html")
            .await
            .unwrap_or_default();
        Ok(Html(body).into_response())
    }
    /// Creates a new instance of the API with the given state.
    pub fn new(state: Arc<ApiState>) -> Self {
        let router = Router::new()
            // Public endpoints
            .route("/api/v1/chains", get(Api::get_chains))
            .route("/api/v1/chains/{chain_id}", get(Api::get_chain))
            .route("/api/v1/login", post(Api::login))
            // Admin endpoints (require session token)
            .route(
                "/api/v1/admin/chains",
                get(Api::admin_get_chains).post(Api::admin_add_chain),
            )
            .route(
                "/api/v1/admin/chains/{chain_id}",
                delete(Api::admin_remove_chain),
            )
            .route(
                "/api/v1/admin/chains/{chain_id}/rpcs",
                post(Api::admin_add_rpc).delete(Api::admin_remove_rpc),
            )
            // RPC proxy
            .route("/{chain_id}", axum::routing::any(Self::rpc_or_spa))
            .with_state(state);

        // Serve the SvelteKit static build from ./dist
        let spa = ServeDir::new("dist").fallback(ServeFile::new("dist/200.html"));
        let router = router.fallback_service(spa);
        Self { router }
    }

    pub fn router(self) -> Router {
        self.router
    }
}
