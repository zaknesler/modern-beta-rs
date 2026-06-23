#[derive(Clone, Debug, serde::Deserialize)]
pub struct OnlinePlayerCountResponse {
    pub online: u32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct OnlinePlayersResponse {
    pub count: u32,
    pub names: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
pub enum ClientType {
    Java,
    Bedrock,
    Beta,
    #[serde(rename = "VR")]
    Vr,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PlayerLocation {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub world: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PlayerProfileResponse {
    pub uuid: Option<String>,
    pub username: Option<String>,
    pub past_usernames: Option<Vec<String>>,
    pub online: bool,
    pub reg_date: Option<String>,
    pub last_seen: Option<String>,
    pub rank_name: Option<String>,
    pub language_code: Option<String>,
    pub played_time_seconds: u32,
    pub client_type: Option<ClientType>,
    pub location: Option<PlayerLocation>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ServerStatsResponse {
    pub total_registered_players: u32,
    pub start_date: Option<String>,
    pub last_reboot: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PlayerPositionEntry {
    pub username: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct WorldPositionsResponse {
    pub world: String,
    pub players: Vec<PlayerPositionEntry>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct WorldResponse {
    pub name: String,
    pub environment: String,
    pub difficulty: String,
    pub time_ticks: i64,
    pub time_formatted: String,
    pub storming: bool,
    pub thundering: bool,
}

impl WorldResponse {
    pub fn weather_state(&self) -> &'static str {
        if self.thundering {
            "Thunderstorm"
        } else if self.storming {
            "Rain"
        } else {
            "Clear"
        }
    }
}
