pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    DecodeError {
        url: String,
        body: String,
        #[source]
        source: serde_json::Error,
    },

    #[error(transparent)]
    ClientBuild(#[from] reqwest::Error),
}
