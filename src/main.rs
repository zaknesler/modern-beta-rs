#![allow(clippy::result_large_err)]

mod api;
mod config;
mod error;
mod state;
mod tray;
mod worker;

use std::process;
use tao::event_loop::EventLoopBuilder;
use tracing::error;
use tracing_subscriber::EnvFilter;

fn main() -> error::AppResult<()> {
    init_tracing();

    let config = match config::AppConfig::load() {
        Ok(config) => config,
        Err(err) => {
            error!(error = %err, "failed to load application config");
            process::exit(1);
        }
    };

    let shared_state = state::SharedAppState::new(state::AppState {
        config,
        ..Default::default()
    });
    let initial_state = shared_state.current();

    let event_loop = EventLoopBuilder::<state::AppEvent>::with_user_event().build();
    let event_proxy = event_loop.create_proxy();

    tray::install_menu_event_handler(event_proxy.clone());

    let tray_app = tray::TrayApp::try_new(initial_state)?;
    worker::spawn_worker(shared_state, event_proxy);

    tray::run(event_loop, tray_app);
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .try_init();
}
