use crate::{
    error::{AppError, AppResult},
    state::AppEvent,
    state::{AppState, SharedAppState},
};
use std::thread;
use tao::event_loop::EventLoopProxy;
use tracing::{error, info, warn};

pub fn spawn_worker(shared_state: SharedAppState, event_proxy: EventLoopProxy<AppEvent>) {
    let builder = thread::Builder::new();
    let config = shared_state.current().config;

    if let Err(err) = builder.spawn(move || {
        info!(
            world = %config.world_name,
            refresh_interval_secs = config.refresh_interval_secs,
            "starting background poll worker"
        );

        if let Err(err) = run_worker(shared_state, event_proxy) {
            error!(error = %err, "background poll worker exited with error");
        }
    }) {
        error!(error = %err, "failed to spawn background poll worker");
    }
}

fn run_worker(
    shared_state: SharedAppState,
    event_proxy: EventLoopProxy<AppEvent>,
) -> AppResult<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(AppError::Runtime)?;

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

            if event_proxy
                .send_event(AppEvent::StateUpdated(new_state))
                .is_err()
            {
                break;
            }
        }

        Ok(())
    })
}
