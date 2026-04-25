use alloy::primitives::utils::UnitsError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Json serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Environment error: {0}")]
    Env(#[from] crate::env::EnvError),

    #[error("Units parsing error: {0}")]
    UnitsParse(#[from] UnitsError),
}

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_not_found_returns_404() {
        let err = ApiError::NotFound("item missing".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_bad_request_returns_400() {
        let err = ApiError::BadRequest("invalid input".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_internal_returns_500() {
        let err = ApiError::Internal("something broke".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_display_not_found() {
        let err = ApiError::NotFound("test".to_string());
        assert_eq!(err.to_string(), "Not found: test");
    }

    #[test]
    fn test_error_display_bad_request() {
        let err = ApiError::BadRequest("bad".to_string());
        assert_eq!(err.to_string(), "Bad request: bad");
    }

    #[test]
    fn test_error_display_internal() {
        let err = ApiError::Internal("oops".to_string());
        assert_eq!(err.to_string(), "Internal error: oops");
    }
}
