#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Failed to fetch players: {0}")]
    FetchError(#[from] reqwest::Error),
}

pub type ApiResult<T> = Result<T, ApiError>;

const PLAYERS_URL: &str = "https://map.modernbeta.org/maps/world/live/players.json";

#[derive(Debug, serde::Deserialize)]
pub struct LivePlayerResponse {
    players: Vec<Player>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Player {
    uuid: String,
    name: String,
    foreign: bool,
    position: Position,
    rotation: Rotation,
}

#[derive(Debug, serde::Deserialize)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Rotation {
    pitch: f64,
    yaw: f64,
    roll: f64,
}

pub async fn get_players() -> ApiResult<LivePlayerResponse> {
    let res = reqwest::get(PLAYERS_URL).await?;
    let json = res.json::<LivePlayerResponse>().await?;
    Ok(json)
}
