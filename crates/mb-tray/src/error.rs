pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid config: {0}")]
    InvalidConfig(String),

    #[error("Could not initialize tracing: {0}")]
    TracingInitError(String),

    #[error("Could not find icon")]
    IconNotFound,

    #[error("user system config directory not found")]
    ConfigDirNotFound,

    #[error(transparent)]
    ApiError(#[from] modern_beta_api::Error),

    #[error(transparent)]
    ImageDecode(#[from] image::ImageError),

    #[error(transparent)]
    BadIcon(#[from] tray_icon::BadIcon),

    #[error(transparent)]
    TrayIcon(#[from] tray_icon::Error),

    #[error(transparent)]
    TrayMenuError(#[from] tray_icon::menu::Error),

    #[error(transparent)]
    ConfigError(#[from] figment::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
}
