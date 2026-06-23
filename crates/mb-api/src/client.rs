use crate::{Error, Result, model};
use serde::de::DeserializeOwned;

const API_BASE_URL: &str = "https://api.modernbeta.org/api/v1";

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    config: ClientConfig,
}

#[derive(Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub world_name: String,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = reqwest::Client::builder().build().map_err(Error::ClientBuild)?;

        Ok(Self { config, client })
    }

    pub async fn get_player_profile(&self, username: &str) -> Result<model::PlayerProfileResponse> {
        self.make_request(format!("{API_BASE_URL}/players/{username}/profile")).await
    }

    pub async fn get_online_player_count(&self) -> Result<model::OnlinePlayerCountResponse> {
        self.make_request(format!("{API_BASE_URL}/server/online")).await
    }

    pub async fn get_online_players(&self) -> Result<model::OnlinePlayersResponse> {
        self.make_request(format!("{API_BASE_URL}/server/online/all")).await
    }

    pub async fn get_server_stats(&self) -> Result<model::ServerStatsResponse> {
        self.make_request(format!("{API_BASE_URL}/server/stats")).await
    }

    pub async fn get_world(&self) -> Result<model::WorldResponse> {
        self.make_request(format!("{API_BASE_URL}/worlds/{}", self.config.world_name))
            .await
    }

    pub async fn get_world_positions(&self) -> Result<model::WorldPositionsResponse> {
        self.make_request(format!(
            "{API_BASE_URL}/worlds/{}/positions",
            self.config.world_name
        ))
        .await
    }

    async fn make_request<T>(&self, url: String) -> Result<T>
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
            .map_err(|source| Error::Request {
                url: url.clone(),
                source,
            })?;

        let status = response.status();
        let body = response.text().await.map_err(|source| Error::Request {
            url: url.clone(),
            source,
        })?;

        if !status.is_success() {
            return Err(Error::HttpStatus { url, status, body });
        }

        serde_json::from_str(&body).map_err(|source| Error::DecodeError { url, body, source })
    }
}
