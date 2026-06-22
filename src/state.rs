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
