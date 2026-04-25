use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Std io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Password cannot be empty")]
    EmptyPassword,

    #[error("Passwords do not match")]
    PasswordMismatch,
}
