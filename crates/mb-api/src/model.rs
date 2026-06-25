#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct OnlinePlayerCountResponse {
    /// The number of players currently online.
    pub online: u32,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct OnlinePlayersResponse {
    /// The total number of players currently online.
    pub count: u32,
    /// The usernames of all currently online players.
    pub names: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
pub enum ClientType {
    Java,
    Bedrock,
    Beta,
    #[serde(rename = "VR")]
    VirtualReality,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct PlayerLocation {
    /// The X coordinate of the player's position.
    pub x: i32,
    /// The Y coordinate of the player's position.
    pub y: i32,
    /// The Z coordinate of the player's position.
    pub z: i32,
    /// The name of the world the player is currently in.
    pub world: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct PlayerProfileResponse {
    /// The player's UUID.
    pub uuid: Option<String>,
    /// The player's current username.
    pub username: Option<String>,
    /// A list of usernames the player has previously used.
    pub past_usernames: Option<Vec<String>>,
    /// Whether the player is currently online.
    pub online: bool,
    /// The date the player registered on the server.
    pub reg_date: Option<String>,
    /// The date and time the player was last seen on the server.
    pub last_seen: Option<String>,
    /// The name of the player's current rank.
    pub rank_name: Option<String>,
    /// The player's language code (e.g. "en").
    pub language_code: Option<String>,
    /// The total number of seconds the player has played on the server.
    pub played_time_seconds: u32,
    /// The type of client the player is using (e.g. Java, Bedrock).
    pub client_type: Option<ClientType>,
    /// The player's current location in the world, if available.
    pub location: Option<PlayerLocation>,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct ServerStatsResponse {
    /// The total number of players registered on the server.
    pub total_registered_players: u32,
    /// The date the server was first started.
    pub start_date: Option<String>,
    /// The date and time of the server's last reboot.
    pub last_reboot: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct PlayerPositionEntry {
    /// The username of the player.
    pub username: String,
    /// The X coordinate of the player's position.
    pub x: i32,
    /// The Y coordinate of the player's position.
    pub y: i32,
    /// The Z coordinate of the player's position.
    pub z: i32,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct WorldPositionsResponse {
    /// The name of the world these positions are from.
    pub world: String,
    /// The list of visible online players and their positions in the world.
    pub players: Vec<PlayerPositionEntry>,
}

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub struct WorldResponse {
    /// The name of the world.
    pub name: String,
    /// The environment type of the world (e.g. normal, nether, the_end).
    pub environment: String,
    /// The difficulty of the world (e.g. peaceful, easy, normal, hard).
    pub difficulty: String,
    /// The current world time expressed in ticks.
    pub time_ticks: i64,
    /// The current world time in a human-readable format.
    pub time_formatted: String,
    /// Whether it is currently raining or snowing in the world.
    pub storming: bool,
    /// Whether there is currently a thunderstorm in the world.
    pub thundering: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherState {
    Thunderstorm,
    Rain,
    Clear,
}

impl WorldResponse {
    /// Get a simplified weather state based on the storming and thundering fields.
    pub fn weather_state(&self) -> WeatherState {
        if self.thundering {
            WeatherState::Thunderstorm
        } else if self.storming {
            WeatherState::Rain
        } else {
            WeatherState::Clear
        }
    }
}

impl std::fmt::Display for WeatherState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherState::Thunderstorm => write!(f, "Thunderstorm"),
            WeatherState::Rain => write!(f, "Rain"),
            WeatherState::Clear => write!(f, "Clear"),
        }
    }
}
