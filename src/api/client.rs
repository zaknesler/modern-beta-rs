use crate::{
    api::model,
    config::AppConfig,
    error::{AppError, AppResult},
};
use serde::de::DeserializeOwned;

const API_BASE_URL: &str = "https://api.modernbeta.org/api/v1";

#[derive(Clone)]
pub struct ApiClient {
    config: AppConfig,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(config: AppConfig) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(AppError::ClientBuild)?;

        Ok(Self { config, client })
    }

    pub async fn get_player_profile(
        &self,
        username: &str,
    ) -> AppResult<model::PlayerProfileResponse> {
        self.make_request(format!("{API_BASE_URL}/players/{username}/profile"))
            .await
    }

    pub async fn get_online_player_count(&self) -> AppResult<model::OnlinePlayerCountResponse> {
        self.make_request(format!("{API_BASE_URL}/server/online"))
            .await
    }

    pub async fn get_online_players(&self) -> AppResult<model::OnlinePlayersResponse> {
        self.make_request(format!("{API_BASE_URL}/server/online/all"))
            .await
    }

    pub async fn get_server_stats(&self) -> AppResult<model::ServerStatsResponse> {
        self.make_request(format!("{API_BASE_URL}/server/stats"))
            .await
    }

    pub async fn get_world(&self) -> AppResult<model::WorldResponse> {
        self.make_request(format!("{API_BASE_URL}/worlds/{}", self.config.world_name))
            .await
    }

    pub async fn get_world_positions(&self) -> AppResult<model::WorldPositionsResponse> {
        self.make_request(format!(
            "{API_BASE_URL}/worlds/{}/positions",
            self.config.world_name
        ))
        .await
    }

    async fn make_request<T>(&self, url: String) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.config.api_key)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|source| AppError::Request {
                url: url.clone(),
                source,
            })?;

        let status = response.status();
        let body = response.text().await.map_err(|source| AppError::Request {
            url: url.clone(),
            source,
        })?;

        if !status.is_success() {
            return Err(AppError::HttpStatus { url, status, body });
        }

        serde_json::from_str(&body).map_err(|source| AppError::DecodeError { url, body, source })
    }
}
