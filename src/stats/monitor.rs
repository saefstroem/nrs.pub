use futures::future::join_all;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use super::{ChainStats, ChainStatus, result::Result};
use crate::{
    rpc::RpcChain,
    stats::{Monitor, error::StatsError},
};

const RPC_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRIES: u32 = 3;
const RETRY_BASE_MS: u64 = 500;
const POLL_INTERVAL: Duration = Duration::from_secs(60);
const HOURLY_SLOTS: usize = 24;

/// Parses a hex string (e.g. "0x1a") into a u64, returning 0 on failure.
fn parse_block_number(hex: &str) -> Result<u64> {
    u64::from_str_radix(hex.trim_start_matches("0x"), 16)
        .map_err(|_| StatsError::InvalidHex(hex.to_string()))
}

/// Returns the current time in seconds since the UNIX epoch.
fn now_epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Monitor {
    pub fn new(rpc_storage: Arc<parking_lot::RwLock<crate::rpc::RpcStorage>>) -> Self {
        Self {
            rpc_storage,
            chain_stats: Arc::new(parking_lot::RwLock::new(ahash::AHashMap::new())),
        }
    }

    /// Probes a single RPC endpoint with retries.
    async fn probe_rpc(
        client: &reqwest::Client,
        url: &str,
        chain_id: u64,
        body: &serde_json::Value,
    ) -> Result<f64> {
        // We retry each RPC up to 3 times
        for attempt in 0..MAX_RETRIES {
            // Start timing
            let start = Instant::now();
            match client.post(url).json(body).send().await {
                Ok(resp) => {
                    let json_resp = resp.json::<serde_json::Value>().await?;
                    // Measure elapsed time for stats
                    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                    let hex = json_resp.get("result").and_then(|v| v.as_str()).ok_or(
                        StatsError::InvalidHex("Could not find viable hex number".to_string()),
                    )?;

                    // Ensure the RPC at least can give us a proper block number
                    parse_block_number(hex)?;

                    // In the future we will need to update this in order to
                    // support non-evm chain. But for now this will do.

                    // Return the probe result.
                    return Ok(elapsed);
                }
                Err(e) => {
                    tracing::warn!(
                        "chain_id: {} rpc: {} request error (attempt {}): {}",
                        chain_id,
                        url,
                        attempt + 1,
                        e
                    );
                }
            }

            // Exponential backoff before retrying
            if attempt < MAX_RETRIES - 1 {
                tokio::time::sleep(Duration::from_millis(RETRY_BASE_MS * (attempt + 1) as u64))
                    .await;
            }
        }

        tracing::error!(
            "chain_id: {} rpc: {} failed after {} attempts",
            chain_id,
            url,
            MAX_RETRIES
        );
        Err(StatsError::RpcUnreachable)
    }

    /// Performs a single round of stats collection for a given chain, updating the internal state.
    pub async fn update_chain_stat(&self, chain: &RpcChain) -> Result<()> {
        let client = reqwest::Client::builder().timeout(RPC_TIMEOUT).build()?;
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });

        // Send request for all rpcs in parallel, collecting Ok/Err per endpoint.
        let results: Vec<Result<f64>> = join_all(
            chain
                .rpcs
                .iter()
                .map(|url| Self::probe_rpc(&client, url, chain.chain_id, &body)),
        )
        .await;

        // Compute number of successful probes, average latency, and error percentage.
        let total_count = results.len() as u32;

        // Accumulate successes and latency in one pass; log individual RPC failures.
        let (success_count, total_latency_ms) =
            results
                .iter()
                .enumerate()
                .fold((0u32, 0.0f64), |(count, latency), (i, probe)| match probe {
                    Ok(ms) => (count + 1, latency + ms),
                    Err(e) => {
                        tracing::warn!(
                            "chain_id: {} rpc[{}] {}: {}",
                            chain.chain_id,
                            i,
                            chain.rpcs[i],
                            e
                        );
                        (count, latency)
                    }
                });

        // Compute the average latency and error percentage, 
        // guarding against division by zero.
        let avg_latency_ms = if success_count > 0 {
            total_latency_ms / success_count as f64
        } else {
            0.0
        };

        // Compute the error pct, treating zero total count as 100% error.
        let error_pct = if total_count > 0 {
            ((total_count - success_count) as f64 / total_count as f64) * 100.0
        } else {
            unreachable!("We should never have zero total count since we \
            only call this with chains that have at least one RPC, and reaching this \
            point with zero total count would indicate a logic error in our code.")
        };

        // A chain is considered "up" if at least one RPC responded successfully.
        let success = success_count > 0;

        // Update chain_stats
        let now = now_epoch_secs();

        // Acquite write lock
        let mut stats = self.chain_stats.write();

        // Get or insert the ChainStats for this chain. If it's a new entry, we initialize it with default values.
        let entry = stats.entry(chain.chain_id).or_insert_with(|| {
            let next_hour = now + 3600;
            ChainStats {
                avg_latency_ms: 0.0,
                error_pct: 100.0,
                success: false,
                hourly_status: std::array::from_fn(|_| ChainStatus::Unknown),
                next_hourly_entry: next_hour,
            }
        });

        // Update the stats entry with the new measurements.
        entry.avg_latency_ms = avg_latency_ms;
        entry.error_pct = error_pct;
        entry.success = success;

        // Roll hourly status if past the deadline
        if now >= entry.next_hourly_entry {
            // Shift left, dropping oldest hour
            entry.hourly_status.rotate_left(1);
            // Write this hour's status into the last slot
            entry.hourly_status[HOURLY_SLOTS - 1] = if success {
                ChainStatus::Up
            } else {
                ChainStatus::Down
            };
            // Schedule next roll
            entry.next_hourly_entry = now + 3600;
        }

        Ok(())
    }

    /// Fail safe method to update all stats for all chains, used for periodic updates.
    pub async fn do_stat_updates(&self) {
        let chains = self.rpc_storage.read().all_chains_owned();
        for chain in &chains {
            if let Err(e) = self.update_chain_stat(chain).await {
                tracing::error!("chain_id: {} error: {}", chain.chain_id, e);
            }
        }
    }

    /// Initializes the Monitor by loading chains
    /// and then periodically updating stats in the background.
    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                self.do_stat_updates().await;
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        });
    }

    /// Retrieves the current stats for a specific chain.
    pub async fn get_chain_stats(&self, chain_id: u64) -> Option<ChainStats> {
        self.chain_stats.read().get(&chain_id).cloned()
    }
}
