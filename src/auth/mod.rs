pub(crate) mod error;
mod result;

use result::Result;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Stores and verifies the admin password as a SHA-256 hex digest.
pub struct PasswordStore {
    path: PathBuf,
}

impl PasswordStore {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Hashes `password` with SHA-256 and writes the hex digest to the pw file.
    pub fn set(&self, password: &str) -> Result<()> {
        let hash = hex::encode(Sha256::digest(password.as_bytes()));
        fs::write(&self.path, hash)?;
        Ok(())
    }

    /// Returns `true` if `attempt` matches the stored password hash.
    pub fn verify(&self, attempt: &str) -> Result<bool> {
        let stored = fs::read_to_string(&self.path)?;
        let hash = hex::encode(Sha256::digest(attempt.as_bytes()));
        Ok(hash == stored.trim())
    }

    /// Prompts for a new password interactively and stores it via `store`.
    /// Called when the binary is started with `--setpass`.
    pub fn setup_password(&self) -> Result<()> {
        // Prompt for password and confirmation, ensuring they match and are not empty.
        let password = rpassword::prompt_password("Enter new admin password: ")?;

        // Enforce non-empty passwords for basic security hygiene.
        if password.is_empty() {
            return Err(crate::auth::error::AuthError::EmptyPassword);
        }

        // Prompt for confirmation and ensure it matches the first entry.
        let confirm = rpassword::prompt_password("Confirm password: ")?;

        // If the passwords don't match, return an error instead of setting a new password.
        if password != confirm {
            return Err(crate::auth::error::AuthError::PasswordMismatch);
        }

        // If we got here, the password is valid and confirmed. Hash and store it.
        self.set(&password)?;
        tracing::info!("Password set successfully");
        Ok(())
    }
}
