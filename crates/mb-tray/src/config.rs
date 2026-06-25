use crate::error::{AppError, AppResult};
use figment::{
    Figment,
    providers::{Format, Toml},
};
use std::{collections::HashSet, time::Duration};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

const PROJECT_DIR: &str = "mb-tray";
const DEFAULT_FILE_NAME: &str = "default.config.toml";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(rust_embed::Embed)]
#[folder = "stubs"]
struct StubAssetDir;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AppConfig {
    pub api_key: String,
    pub world_name: String,
    pub refresh_interval_secs: u64,
    pub favorite_players: HashSet<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            world_name: "world".to_string(),
            refresh_interval_secs: 30,
            favorite_players: HashSet::new(),
        }
    }
}

impl AppConfig {
    pub fn load() -> AppResult<AppConfig> {
        Self::init_config_file()?;

        let config_dir = Self::get_config_dir()?;

        let config = Figment::new()
            .merge(Toml::string(std::str::from_utf8(
                Self::get_default_data().as_ref(),
            )?))
            .merge(Toml::file(
                config_dir
                    .join(CONFIG_FILE_NAME)
                    .to_str()
                    .expect("path should be valid unicode"),
            ))
            .extract::<AppConfig>()?;

        config.validate()
    }

    fn get_default_data() -> Vec<u8> {
        let default = StubAssetDir::get(DEFAULT_FILE_NAME).expect("default.toml stub should exist");
        default.data.as_ref().to_owned()
    }

    fn get_config_dir() -> AppResult<PathBuf> {
        directories::ProjectDirs::from("", "", PROJECT_DIR)
            .map(|dirs| dirs.config_dir().to_path_buf())
            .ok_or_else(|| AppError::ConfigDirNotFound)
    }

    /// Initialize config directory and config.toml
    fn init_config_file() -> AppResult<PathBuf> {
        let config_dir = Self::init_config_dir()?;

        // Create local config if it doesn't exist
        let local_config_file = config_dir.join(CONFIG_FILE_NAME);
        let exists = local_config_file.try_exists()?;

        let path_str = local_config_file.clone();
        let path_str = path_str.to_str().unwrap_or_default();

        if !exists {
            let mut local_config = File::create(local_config_file)?;
            local_config.write_all(Self::get_default_data().as_ref())?;
        } else {
            tracing::info!("Config file already exists at {}", &path_str)
        }

        Ok(config_dir)
    }

    /// Initialize config directory
    fn init_config_dir() -> AppResult<PathBuf> {
        let config_dir = Self::get_config_dir()?;

        // Create project config directory if it doesn't exist
        fs::create_dir_all(config_dir.clone())?;

        Ok(config_dir)
    }

    fn validate(self) -> AppResult<Self> {
        if self.api_key.trim().is_empty() {
            return Err(AppError::InvalidConfig(
                "`api_key` must not be empty".to_string(),
            ));
        }

        if self.world_name.trim().is_empty() {
            return Err(AppError::InvalidConfig(
                "`world_name` must not be empty".to_string(),
            ));
        }

        if self.refresh_interval_secs == 0 {
            return Err(AppError::InvalidConfig(
                "`refresh_interval_secs` must be greater than 0".to_string(),
            ));
        }

        Ok(self)
    }

    pub fn refresh_interval(&self) -> Duration {
        Duration::from_secs(self.refresh_interval_secs)
    }
}
