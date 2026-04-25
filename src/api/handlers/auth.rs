use std::sync::Arc;
use std::time::Duration;

use axum::{Json, extract::State, response::IntoResponse};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

use crate::api::{Api, ApiState};
use crate::api::error::ApiError;
use crate::api::result::Result;

#[derive(Deserialize)]
pub struct LoginRequest {
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

impl Api {
    pub async fn login(
        State(state): State<Arc<ApiState>>,
        Json(body): Json<LoginRequest>,
    ) -> Result<impl IntoResponse> {
        // Only allow login if a password is configured
        if !state.pw_store.exists() {
            return Err(ApiError::Unauthorized(
                "No password configured. Start the server with --setpass to set one.".into(),
            ));
        }

        // Verify the provided password
        let verified = state
            .pw_store
            .verify(&body.password)
            .map_err(|e| ApiError::Internal(e.to_string()))?;

        // If verification fails, return Unauthorized
        if !verified {
            return Err(ApiError::Unauthorized("Invalid password".into()));
        }

        // Generate a random session token
        let mut bytes = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut bytes);
        let token = hex::encode(bytes);

        // Store token with 1-hour expiry
        *state.session.write() = Some((
            token.clone(),
            Instant::now() + Duration::from_secs(3600),
        ));

        Ok(Json(LoginResponse { token }))
    }
}
