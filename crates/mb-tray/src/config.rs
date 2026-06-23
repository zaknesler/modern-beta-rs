use crate::error::{AppError, AppResult};
use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use std::{collections::HashSet, time::Duration};

const CONFIG_FILE_NAME: &str = "mb-tray.toml";

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
    pub fn load() -> AppResult<Self> {
        let config: Self = Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file(CONFIG_FILE_NAME))
            .extract()
            .map_err(AppError::ConfigLoad)?;

        config.validate()
    }

    pub fn refresh_interval(&self) -> Duration {
        Duration::from_secs(self.refresh_interval_secs)
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
}
