use crate::auth::error::AuthError;


pub type Result<T> = std::result::Result<T, AuthError>;