use std::sync::{Arc, Mutex};

use crate::config::AppConfig;

#[derive(Clone, Default)]
pub struct SharedAppState(Arc<Mutex<AppState>>);

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub config: AppConfig,
    pub data: Option<ResponseData>,
}

#[derive(Clone, Debug)]
pub struct ResponseData {
    pub online_players: crate::api::OnlinePlayersResponse,
    pub world: crate::api::WorldResponse,
}

pub enum OnlinePlayersState {
    Loading,
    Empty,
    Unavailable,
    Loaded(Vec<String>),
}

impl AppState {
    pub fn player_count(&self) -> Option<usize> {
        self.data.as_ref().map(|data| data.online_players.count)
    }

    pub fn online_players(&self) -> OnlinePlayersState {
        match self.data.as_ref() {
            None => OnlinePlayersState::Loading,
            Some(data) if data.online_players.count == 0 => OnlinePlayersState::Empty,
            Some(data) => match data.online_players.names.as_ref() {
                Some(names) => OnlinePlayersState::Loaded(names.clone()),
                None => OnlinePlayersState::Unavailable,
            },
        }
    }

    pub fn online_favorite_count(&self) -> usize {
        match self.online_players() {
            OnlinePlayersState::Loaded(names) => names
                .iter()
                .filter(|name| self.config.favorite_players.contains(*name))
                .count(),
            _ => 0,
        }
    }

    pub fn world(&self) -> Option<&crate::api::WorldResponse> {
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

pub enum AppEvent {
    Menu(tray_icon::menu::MenuEvent),
    StateUpdated(AppState),
}
