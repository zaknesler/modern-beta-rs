#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid config: {0}")]
    InvalidConfig(String),

    #[error("Could not initialize tracing: {0}")]
    TracingInitError(String),

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
    ConfigLoad(#[from] figment::Error),

    #[error(transparent)]
    Runtime(#[from] std::io::Error),

    #[error(transparent)]
    ClientBuild(#[from] reqwest::Error),

    #[error(transparent)]
    ImageDecode(#[from] image::ImageError),

    #[error(transparent)]
    BadIcon(#[from] tray_icon::BadIcon),

    #[error(transparent)]
    TrayIcon(#[from] tray_icon::Error),

    #[error(transparent)]
    TrayMenuError(#[from] tray_icon::menu::Error),
}

pub type AppResult<T> = Result<T, AppError>;
