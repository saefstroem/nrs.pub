pub(crate) mod error;
mod result;

use result::Result;
use error::RpcError;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A raw chain entry
#[derive(Deserialize)]
struct RawChain {
    name: String,
    #[serde(default)]
    rpc: Vec<String>,
    #[serde(rename = "chainId", default)]
    chain_id: u64,
}

/// Runtime representation of a chain with its RPC endpoints
#[derive(Clone, Serialize)]
pub struct RpcChain {
    pub name: String,
    pub chain_id: u64,
    pub rpcs: Vec<String>,
}

impl std::fmt::Debug for RpcChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcChain")
            .field("name", &self.name)
            .field("chain_id", &self.chain_id)
            .field("rpcs", &self.rpcs)
            .finish()
    }
}

/// In-memory storage of all chain RPC configurations
pub struct RpcStorage {
    chains: HashMap<u64, RpcChain>,
    /// Full raw JSON entries — used to persist mutations while preserving all original fields.
    raw: Vec<serde_json::Value>,
    rpcs_path: PathBuf,
}

impl RpcStorage {
    /// Load and parse rpcs.json into an RpcStorage instance.
    /// Filters out chains without valid chain IDs and HTTP(S) RPC entries.
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;

        // Keep raw JSON for persistence fidelity (all original fields preserved)
        let raw: Vec<serde_json::Value> = serde_json::from_str(&data)?;

        // Parse typed structs for runtime use
        let raw_chains: Vec<RawChain> = serde_json::from_str(&data)?;

        let mut chains = HashMap::new();
        for rc in raw_chains {
            let rpcs: Vec<String> = rc
                .rpc
                .into_iter()
                .filter(|url| url.starts_with("http://") || url.starts_with("https://"))
                .collect();
            if rpcs.is_empty() {
                continue;
            }
            chains.insert(
                rc.chain_id,
                RpcChain {
                    name: rc.name,
                    chain_id: rc.chain_id,
                    rpcs,
                },
            );
        }

        tracing::info!("Loaded {} chains with HTTP RPC endpoints", chains.len());
        Ok(Self { chains, raw, rpcs_path: path.to_path_buf() })
    }

    /// Look up a chain by ID, returning a clone (safe to use after releasing the lock).
    pub fn get_chain_cloned(&self, chain_id: u64) -> Option<RpcChain> {
        self.chains.get(&chain_id).cloned()
    }

    /// Return all chains as owned clones (safe to use after releasing the lock).
    pub fn all_chains_owned(&self) -> Vec<RpcChain> {
        let mut v: Vec<RpcChain> = self.chains.values().cloned().collect();
        v.sort_by_key(|c| c.chain_id);
        v
    }

    /// Return lightweight info for all chains (for the public /chains endpoint).
    pub fn all_chains(&self) -> Vec<ChainInfo> {
        let mut chains: Vec<ChainInfo> = self
            .chains
            .values()
            .map(|c| ChainInfo {
                name: c.name.clone(),
                chain_id: c.chain_id,
                redundancy_set: c.rpcs.len() as u64,
            })
            .collect();
        chains.sort_by_key(|c| c.chain_id);
        chains
    }

    /// Add a new RPC URL to an existing chain and persist to disk.
    pub fn add_rpc(&mut self, chain_id: u64, url: String) -> Result<()> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(RpcError::InvalidUrl(url));
        }
        let chain = self.chains.get_mut(&chain_id).ok_or(RpcError::NotFound(chain_id))?;
        if !chain.rpcs.contains(&url) {
            chain.rpcs.push(url.clone());
        }
        if let Some(entry) = self.raw.iter_mut().find(|v| v["chainId"].as_u64() == Some(chain_id)) {
            if let Some(arr) = entry["rpc"].as_array_mut() {
                let already = arr.iter().any(|v| {
                    v.as_str() == Some(&url)
                        || v.get("url").and_then(|u| u.as_str()) == Some(&url)
                });
                if !already {
                    arr.push(serde_json::Value::String(url));
                }
            }
        }
        self.save()
    }

    /// Remove an RPC URL from an existing chain and persist to disk.
    pub fn remove_rpc(&mut self, chain_id: u64, url: &str) -> Result<()> {
        let chain = self.chains.get_mut(&chain_id).ok_or(RpcError::NotFound(chain_id))?;
        chain.rpcs.retain(|u| u != url);
        if let Some(entry) = self.raw.iter_mut().find(|v| v["chainId"].as_u64() == Some(chain_id)) {
            if let Some(arr) = entry["rpc"].as_array_mut() {
                arr.retain(|v| {
                    let matches = v.as_str() == Some(url)
                        || v.get("url").and_then(|u| u.as_str()) == Some(url);
                    !matches
                });
            }
        }
        self.save()
    }

    /// Add a brand-new chain and persist to disk. Errors if chain_id already exists.
    pub fn add_chain(&mut self, chain_id: u64, name: String, rpcs: Vec<String>) -> Result<()> {
        if self.chains.contains_key(&chain_id) {
            return Err(RpcError::AlreadyExists(chain_id));
        }
        let valid_rpcs: Vec<String> = rpcs
            .into_iter()
            .filter(|u| u.starts_with("http://") || u.starts_with("https://"))
            .collect();
        let raw_rpcs: Vec<serde_json::Value> =
            valid_rpcs.iter().map(|u| serde_json::Value::String(u.clone())).collect();
        let raw_entry = serde_json::json!({
            "name": name,
            "chainId": chain_id,
            "rpc": raw_rpcs,
            "faucets": [],
            "nativeCurrency": { "name": "", "symbol": "", "decimals": 18 },
            "infoURL": "",
            "shortName": "",
            "isTestnet": false
        });
        self.raw.push(raw_entry);
        self.chains.insert(
            chain_id,
            RpcChain { name, chain_id, rpcs: valid_rpcs },
        );
        self.save()
    }

    /// Remove a chain entirely and persist to disk.
    pub fn remove_chain(&mut self, chain_id: u64) -> Result<()> {
        self.chains.remove(&chain_id);
        self.raw.retain(|v| v["chainId"].as_u64() != Some(chain_id));
        self.save()
    }

    /// Write the raw JSON back to rpcs_path, preserving all original fields.
    fn save(&self) -> Result<()> {
        let json = serde_json::to_string(&self.raw)?;
        std::fs::write(&self.rpcs_path, json)?;
        Ok(())
    }
}

/// Lightweight chain info returned by GET /api/v1/chains
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChainInfo {
    pub name: String,
    pub chain_id: u64,
    pub redundancy_set: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    fn rpcs_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("rpcs.json")
    }

    #[test]
    fn test_load_rpcs_json() -> TestResult {
        let storage = RpcStorage::load(&rpcs_path())?;
        assert!(!storage.chains.is_empty(), "Should load at least one chain");
        Ok(())
    }

    #[test]
    fn test_get_chain_ethereum() -> TestResult {
        let storage = RpcStorage::load(&rpcs_path())?;
        let eth = storage.get_chain_cloned(1).ok_or("ethereum chain (chain_id=1) not found")?;
        assert_eq!(eth.chain_id, 1);
        assert!(!eth.rpcs.is_empty(), "Ethereum should have RPCs");
        for rpc in &eth.rpcs {
            assert!(
                rpc.starts_with("http://") || rpc.starts_with("https://"),
                "RPC should be HTTP(S): {}",
                rpc
            );
        }
        Ok(())
    }

    #[test]
    fn test_get_chain_nonexistent() -> TestResult {
        let storage = RpcStorage::load(&rpcs_path())?;
        assert!(storage.get_chain_cloned(99999999999).is_none());
        Ok(())
    }

    #[test]
    fn test_all_chains() -> TestResult {
        let storage = RpcStorage::load(&rpcs_path())?;
        let chains = storage.all_chains();
        assert!(!chains.is_empty());
        assert!(chains.iter().any(|c| c.chain_id == 1));
        Ok(())
    }

    #[test]
    fn test_no_wss_rpcs_loaded() -> TestResult {
        let storage = RpcStorage::load(&rpcs_path())?;
        for chain in storage.chains.values() {
            for rpc in &chain.rpcs {
                assert!(
                    !rpc.starts_with("wss://"),
                    "Found wss:// RPC in chain {} (chain_id={}): {}",
                    chain.name,
                    chain.chain_id,
                    rpc
                );
            }
        }
        Ok(())
    }
}
