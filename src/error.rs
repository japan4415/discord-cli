use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("API error (status {status}): {message}")]
    Api { status: u16, message: String },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
