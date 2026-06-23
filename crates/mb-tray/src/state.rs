use crate::config::AppConfig;
use std::sync::{Arc, Mutex};

pub enum AppEvent {
    Menu(tray_icon::menu::MenuEvent),
    StateUpdated(AppState),
}

#[derive(Clone, Default)]
pub struct SharedAppState(Arc<Mutex<AppState>>);

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub config: AppConfig,
    pub data: Option<ResponseData>,
}

#[derive(Clone, Debug)]
pub struct ResponseData {
    pub online_players: modern_beta_api::model::OnlinePlayersResponse,
    pub world: modern_beta_api::model::WorldResponse,
}

pub enum OnlinePlayersState {
    Loading,
    Empty,
    Loaded(Vec<String>),
}

impl AppState {
    pub fn online_players_count(&self) -> Option<u32> {
        self.data.as_ref().map(|data| data.online_players.count)
    }

    pub fn online_players(&self) -> OnlinePlayersState {
        match self.data.as_ref() {
            None => OnlinePlayersState::Loading,
            Some(data) if data.online_players.count == 0 => OnlinePlayersState::Empty,
            Some(data) => OnlinePlayersState::Loaded(data.online_players.names.clone()),
        }
    }

    pub fn online_favorite_players_count(&self) -> Option<usize> {
        match self.online_players() {
            OnlinePlayersState::Loaded(names) => Some(
                names.iter().filter(|name| self.config.favorite_players.contains(*name)).count(),
            ),
            _ => None,
        }
    }

    pub fn world(&self) -> Option<&modern_beta_api::model::WorldResponse> {
        self.data.as_ref().map(|data| &data.world)
    }
}

impl SharedAppState {
    pub fn new(initial_state: AppState) -> Self {
        Self(Arc::new(Mutex::new(initial_state)))
    }

    pub fn current(&self) -> AppState {
        match self.0.lock() {
            Ok(state) => state.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }

    pub fn set(&self, updated: AppState) {
        let mut state = match self.0.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };

        *state = updated;
    }
}
