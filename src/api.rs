use crate::error::{AppError, AppResult};

const API_BASE_URL: &str = "https://api.modernbeta.org/api/v1";

#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    world_name: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct OnlinePlayersResponse {
    pub count: u32,
    pub names: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct WorldResponse {
    pub storming: bool,
    pub thundering: bool,
}

impl ApiClient {
    pub fn new(api_key: String, world_name: String) -> AppResult<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-API-Key",
            reqwest::header::HeaderValue::from_str(&api_key).map_err(|_| {
                AppError::InvalidConfig("`api_key` contains invalid header characters".to_string())
            })?,
        );
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(AppError::ClientBuild)?;

        Ok(Self { client, world_name })
    }

    pub async fn get_online_players(&self) -> AppResult<OnlinePlayersResponse> {
        let url = format!("{API_BASE_URL}/server/online/all");
        let response = self
            .client
            .get(&url)
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

        serde_json::from_str(&body).map_err(|source| AppError::Decode { url, body, source })
    }

    pub async fn get_world(&self) -> AppResult<WorldResponse> {
        let url = format!("{API_BASE_URL}/worlds/{}", self.world_name);
        let response = self
            .client
            .get(&url)
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

        serde_json::from_str(&body).map_err(|source| AppError::Decode { url, body, source })
    }
}

pub fn weather_text(world: &WorldResponse) -> String {
    if world.thundering {
        "Thunderstorm".to_string()
    } else if world.storming {
        "Rain".to_string()
    } else {
        "Clear".to_string()
    }
}
