use std::sync::{Arc, Mutex};

use crate::config::AppConfig;

#[derive(Clone, Default)]
pub struct SharedAppState(Arc<Mutex<AppState>>);

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub config: AppConfig,
    pub player_count: Option<u32>,
    pub player_names: Vec<String>,
    pub weather_text: Option<String>,
}

impl AppState {
    pub fn player_count(&self) -> Option<u32> {
        self.player_count
    }

    pub fn has_loaded_players(&self) -> bool {
        self.player_count.is_some()
    }

    pub fn player_names(&self) -> &[String] {
        &self.player_names
    }

    pub fn weather_text(&self) -> Option<&str> {
        self.weather_text.as_deref()
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
