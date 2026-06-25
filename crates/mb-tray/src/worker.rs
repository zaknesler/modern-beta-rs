use crate::{
    error::AppResult,
    state::{AppState, SharedAppState},
};
use std::{sync::mpsc, thread};
use tracing::{error, info, warn};

pub struct Worker {
    shared_state: SharedAppState,
    tx: mpsc::Sender<AppState>,
}

impl Worker {
    pub fn new(shared_state: SharedAppState, tx: mpsc::Sender<AppState>) -> Self {
        Self { shared_state, tx }
    }

    pub fn spawn(&self) {
        let config = self.shared_state.current().config;
        let worker = Self {
            shared_state: self.shared_state.clone(),
            tx: self.tx.clone(),
        };

        if let Err(err) = thread::Builder::new().spawn(move || {
            info!(
                world = %config.world_name,
                refresh_interval_secs = config.refresh_interval_secs,
                "starting background poll worker"
            );

            if let Err(err) = worker.run() {
                error!(error = %err, "background poll worker exited with error");
            }
        }) {
            error!(error = %err, "failed to spawn background poll worker");
        }
    }

    fn run(&self) -> AppResult<()> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let shared_state = self.shared_state.clone();
        let tx = self.tx.clone();

        runtime.block_on(async move {
            let mut current_state = shared_state.current();

            let api = modern_beta_api::Client::new(modern_beta_api::ClientConfig {
                api_key: current_state.config.api_key.clone(),
                world_name: current_state.config.world_name.clone(),
            })?;
            let mut interval = tokio::time::interval(current_state.config.refresh_interval());
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                interval.tick().await;

                let new_state = match tokio::try_join!(api.get_online_players(), api.get_world()) {
                    Ok((players, world)) => AppState {
                        config: current_state.config.clone(),
                        data: Some(crate::state::ResponseData {
                            online_players: players,
                            world,
                        }),
                    },
                    Err(err) => {
                        warn!(error = %err, "api fetch failed; keeping last successful state");
                        current_state.clone()
                    }
                };

                current_state = new_state.clone();
                shared_state.set(new_state.clone());

                if tx.send(new_state).is_err() {
                    break;
                }
            }

            Ok(())
        })
    }
}
