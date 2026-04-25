use std::sync::Arc;
use std::time::Duration;

use ahash::AHashSet;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use crate::api::result::Result;
use crate::api::{Api, ApiState, error::ApiError};

impl Api {
    /// POST /:chain_id/:api_key
    /// Validates the API key, finds a healthy RPC for the chain via random selection
    /// and returns a 307 redirect to the RPC URL.
    pub async fn rpc_proxy(
        State(state): State<Arc<ApiState>>,
        Path(chain_id): Path<u64>,
    ) -> Result<Response> {
        // Look up chain by network ID (clone immediately to release the read lock)
        let chain = state.rpc_storage.read().get_chain_cloned(chain_id).ok_or_else(|| {
            ApiError::NotFound(format!("Chain with chain_id {} not found", chain_id))
        })?;

        // Check if there are any RPCs available for this chain
        if chain.rpcs.is_empty() {
            return Err(ApiError::Internal(
                "No RPCs available for this chain".to_string(),
            ));
        }

        let total = chain.rpcs.len();

        // Create an HTTP client with a timeout for health checks
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .map_err(|e| ApiError::Internal(format!("HTTP client error: {}", e)))?;

        let mut candidate_rpcs = AHashSet::from_iter(chain.rpcs.iter().collect::<Vec<&String>>());

        while !candidate_rpcs.is_empty() {
            // The random seed is added to prevent attacks like the KempDAO
            // attack where attacker tries to predict the next RPC URL and DDoS it
            // or compromise it in any way.
            let random_seed = rand::random::<u64>();
            let idx = random_seed as usize % candidate_rpcs.len();
            let rpc_url = &chain.rpcs[idx];
            match client
                .post(rpc_url)
                .header("Content-Type", "application/json")
                .body(r#"{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}"#)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        "Location",
                        HeaderValue::from_str(rpc_url)
                            .map_err(|_| ApiError::Internal("Invalid RPC URL".to_string()))?,
                    );
                    return Ok((StatusCode::TEMPORARY_REDIRECT, headers).into_response());
                }
                Ok(resp) => {
                    tracing::warn!(
                        "RPC {} returned status {} for chain_id {}",
                        rpc_url,
                        resp.status(),
                        chain_id
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        "RPC {} health check failed for chain_id {}: {}",
                        rpc_url,
                        chain_id,
                        e
                    );
                    candidate_rpcs.remove(rpc_url);
                }
            }
        }

        // All RPCs failed
        Err(ApiError::Internal(format!(
            "All {} RPCs are down for chain_id {}",
            total, chain_id
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::path::PathBuf;
    use std::sync::Arc;
    use axum::http::StatusCode;
    use serde_json::json;

    use crate::api::{Api, ApiState};
    use crate::auth::PasswordStore;
    use crate::rpc::RpcStorage;
    use crate::stats::Monitor;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    const TEST_API_KEY: &str = "deadbeefcafebabe1234567890abcdef";

    fn rpcs_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("rpcs.json")
    }

    fn temp_db_path(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join("autorpc_test");
        let _ = std::fs::create_dir_all(&dir);
        dir.join(format!("{}.redb", name))
    }

    async fn setup_test_server(
        db_name: &str,
    ) -> std::result::Result<(SocketAddr, PathBuf), Box<dyn std::error::Error>> {
        let db_path = temp_db_path(db_name);
        let _ = std::fs::remove_file(&db_path);

        let rpc_storage = Arc::new(parking_lot::RwLock::new(RpcStorage::load(&rpcs_path())?));
        let monitor = Arc::new(Monitor::new(rpc_storage.clone()));
        let password_store = Arc::new(PasswordStore::new(&PathBuf::from("test_pw")));
        let state = Arc::new(ApiState::new(rpc_storage, monitor, password_store));
        let app = Api::new(state).router();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("Server error: {}", e);
            }
        });

        Ok((addr, db_path))
    }

    #[tokio::test]
    async fn test_rpc_proxy_returns_307_redirect() -> TestResult {
        let (addr, db_path) = setup_test_server("test_rpc_proxy_307").await?;

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let resp = client
            .post(format!("http://{}/1/{}", addr, TEST_API_KEY)) // chain_id 1 = Ethereum
            .send()
            .await?;

        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
        let location = resp
            .headers()
            .get("Location")
            .ok_or("missing Location header")?
            .to_str()?;
        assert!(
            location.starts_with("https://"),
            "Location should be an HTTPS RPC URL, got: {}",
            location
        );

        let _ = std::fs::remove_file(&db_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_rpc_proxy_invalid_api_key() -> TestResult {
        let (addr, db_path) = setup_test_server("test_rpc_proxy_invalid_key").await?;

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let resp = client
            .post(format!("http://{}/1/invalid_key_12345", addr)) // chain_id 1
            .send()
            .await?;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let _ = std::fs::remove_file(&db_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_rpc_proxy_unknown_chain() -> TestResult {
        let (addr, db_path) = setup_test_server("test_rpc_proxy_unknown_chain").await?;

        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let resp = client
            .post(format!("http://{}/99999999999/{}", addr, TEST_API_KEY))
            .send()
            .await?;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let _ = std::fs::remove_file(&db_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_eth_get_block_by_number() -> TestResult {
        let (addr, db_path) = setup_test_server("test_e2e_eth_getblock").await?;

        // Step 1: POST to get the redirect
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let resp = client
            .post(format!("http://{}/1/{}", addr, TEST_API_KEY)) // chain_id 1 = Ethereum
            .send()
            .await?;

        assert_eq!(resp.status(), StatusCode::TEMPORARY_REDIRECT);
        let rpc_url = resp
            .headers()
            .get("Location")
            .ok_or("missing Location header")?
            .to_str()?
            .to_string();

        // Step 2: Send eth_getBlockByNumber to the RPC URL
        let rpc_client = reqwest::Client::new();
        let rpc_resp = rpc_client
            .post(&rpc_url)
            .json(&json!({
                "jsonrpc": "2.0",
                "method": "eth_getBlockByNumber",
                "params": ["latest", false],
                "id": 1
            }))
            .send()
            .await?;

        assert!(
            rpc_resp.status().is_success(),
            "RPC call failed with status: {}",
            rpc_resp.status()
        );

        let body: serde_json::Value = rpc_resp.json().await?;
        assert!(body.get("result").is_some(), "Should have a result field");
        assert!(
            body["result"].get("number").is_some(),
            "Block should have a number field, got: {}",
            body
        );

        let _ = std::fs::remove_file(&db_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_chains_endpoint() -> TestResult {
        let (addr, db_path) = setup_test_server("test_get_chains").await?;

        let resp = reqwest::get(format!("http://{}/api/v1/chains", addr)).await?;

        assert!(resp.status().is_success());
        let chains: Vec<serde_json::Value> = resp.json().await?;
        assert!(!chains.is_empty(), "Should return chains");

        // Check ethereum is present
        let eth = chains.iter().find(|c| c["chain_id"] == 1);
        assert!(
            eth.is_some(),
            "Ethereum (chain_id=1) should be in the chains list"
        );

        let _ = std::fs::remove_file(&db_path);
        Ok(())
    }
}
