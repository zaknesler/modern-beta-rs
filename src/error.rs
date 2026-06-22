#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to load config: {0}")]
    ConfigLoad(#[source] figment::Error),

    #[error("Invalid config: {0}")]
    InvalidConfig(String),

    #[error("Failed to create Tokio runtime: {0}")]
    Runtime(#[source] std::io::Error),

    #[error("Failed to build API client: {0}")]
    ClientBuild(#[source] reqwest::Error),

    #[error("Request to {url} failed: {source}")]
    Request {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Request to {url} returned {status}: {body}")]
    HttpStatus {
        url: String,
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to parse response from {url}: {source}; body: {body}")]
    Decode {
        url: String,
        body: String,
        #[source]
        source: serde_json::Error,
    },
}

pub type AppResult<T> = Result<T, AppError>;
