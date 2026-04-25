use std::env;
use std::sync::LazyLock;

use thiserror::Error;

/// Validates environment variables on startup
#[derive(Debug)]
pub struct EnvConfig {
    pub rpcs_path: String,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("Environment variable {0} is missing or invalid: {1}")]
    InvalidEnv(String, String),
}

impl EnvConfig {
    pub fn load() -> Result<Self, EnvError> {
        let rpcs_path = env::var("RPCS_PATH").unwrap_or_else(|_| "rpcs.json".to_string());
        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|e| {
                EnvError::InvalidEnv("PORT".to_string(), format!("Invalid PORT: {}", e))
            })?;

        Ok(Self { rpcs_path, port })
    }
}

pub static CONFIG: LazyLock<EnvConfig> = LazyLock::new(|| {
    EnvConfig::load().unwrap_or_else(|e| panic!("Failed to load env config: {}", e))
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Env tests must run serially because they modify process env vars
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    /// # Safety: Env var manipulation is unsafe in Rust 2024 edition.
    /// Tests are serialized via ENV_MUTEX to avoid data races.
    fn with_env_vars<F, R>(vars: &[(&str, &str)], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let originals: Vec<_> = vars
            .iter()
            .map(|(k, v)| {
                let orig = env::var(k).ok();
                unsafe { env::set_var(k, v) };
                (*k, orig)
            })
            .collect();

        let result = f();

        for (k, orig) in originals {
            match orig {
                Some(v) => unsafe { env::set_var(k, v) },
                None => unsafe { env::remove_var(k) },
            }
        }
        result
    }

    fn without_env_vars<F, R>(vars: &[&str], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let originals: Vec<_> = vars
            .iter()
            .map(|k| {
                let orig = env::var(k).ok();
                unsafe { env::remove_var(k) };
                (*k, orig)
            })
            .collect();

        let result = f();

        for (k, orig) in originals {
            if let Some(v) = orig {
                unsafe { env::set_var(k, v) };
            }
        }
        result
    }

    #[test]
    fn test_load_missing_oracle_interval() {
        without_env_vars(&["PRICE_ORACLE_UPDATE_INTERVAL_SECS"], || {
            unsafe { env::set_var("USD_PRICE_PER_MONTH", "10") };
            let result = EnvConfig::load();
            assert!(result.is_err());
            if let Err(err) = result {
                assert!(
                    err.to_string()
                        .contains("PRICE_ORACLE_UPDATE_INTERVAL_SECS")
                );
            }
            unsafe { env::remove_var("USD_PRICE_PER_MONTH") };
        });
    }

    #[test]
    fn test_load_missing_usd_price() {
        without_env_vars(&["USD_PRICE_PER_MONTH"], || {
            unsafe { env::set_var("PRICE_ORACLE_UPDATE_INTERVAL_SECS", "60") };
            let result = EnvConfig::load();
            assert!(result.is_err());
            if let Err(err) = result {
                assert!(err.to_string().contains("USD_PRICE_PER_MONTH"));
            }
            unsafe { env::remove_var("PRICE_ORACLE_UPDATE_INTERVAL_SECS") };
        });
    }

    #[test]
    fn test_load_invalid_oracle_interval_not_u64() {
        with_env_vars(
            &[
                ("PRICE_ORACLE_UPDATE_INTERVAL_SECS", "not_a_number"),
                ("USD_PRICE_PER_MONTH", "10"),
            ],
            || {
                let result = EnvConfig::load();
                assert!(result.is_err());
                if let Err(err) = result {
                    assert!(
                        err.to_string()
                            .contains("PRICE_ORACLE_UPDATE_INTERVAL_SECS")
                    );
                }
            },
        );
    }

    #[test]
    fn test_load_invalid_usd_price_not_u64() {
        with_env_vars(
            &[
                ("PRICE_ORACLE_UPDATE_INTERVAL_SECS", "60"),
                ("USD_PRICE_PER_MONTH", "abc"),
            ],
            || {
                let result = EnvConfig::load();
                assert!(result.is_err());
                if let Err(err) = result {
                    assert!(err.to_string().contains("USD_PRICE_PER_MONTH"));
                }
            },
        );
    }

    #[test]
    fn test_env_error_display() {
        let err = EnvError::InvalidEnv("FOO".to_string(), "missing".to_string());
        assert_eq!(
            err.to_string(),
            "Environment variable FOO is missing or invalid: missing"
        );
    }
}
