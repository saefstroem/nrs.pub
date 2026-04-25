mod error;
mod result; 
mod monitor;

use std::sync::Arc;
use ahash::AHashMap;
use parking_lot::RwLock;
use serde::Serialize;
use crate::rpc::RpcStorage;


#[repr(u8)]
#[derive(Clone, Copy, Serialize)]
/// Status of a chain at a given time, used for historical tracking and health assessment.
pub enum ChainStatus {
    Up,
    Down,
    Unknown,
}

#[derive(Clone, Copy,Serialize)]
pub struct ChainStats {
    /// Average latency in milliseconds over its redundancy set
    pub avg_latency_ms: f64,
    /// Per request error percentage (0-100)
    pub error_pct: f64,
    /// Whether the chain is currently considered healthy (based on recent stats)
    pub success: bool,
    /// Historical status for the past 24 hours (one entry per hour)
    pub hourly_status: [ChainStatus; 24],
    /// Next hourly status update timestamp (for scheduling)
    pub next_hourly_entry: u64,
}

pub struct Monitor {
    rpc_storage: Arc<RwLock<RpcStorage>>,
    chain_stats: Arc<RwLock<AHashMap<u64, ChainStats>>>,
}
