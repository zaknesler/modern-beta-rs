#![allow(clippy::result_large_err)]

mod config;
mod error;
mod state;
mod tray;
mod worker;

use std::{process, sync::mpsc, time::Duration};
use tracing::error;
use tracing_subscriber::EnvFilter;

fn main() -> error::AppResult<()> {
    init_tracing()?;

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

    mb_ui::run(move |app| {
        let mut tray_app = match tray::TrayApp::try_new(initial_state) {
            Ok(app) => app,
            Err(err) => {
                error!(error = %err, "failed to create tray");
                return;
            }
        };

        if let Err(err) = tray_app.initialize() {
            error!(error = %err, "failed to initialize tray icon");
            return;
        }

        let (tx, rx) = mpsc::channel::<state::AppState>();
        worker::Worker::new(shared_state, tx).spawn();

        app.spawn(async move |cx| {
            loop {
                // Wait 200ms between polls
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;

                while let Ok(new_state) = rx.try_recv() {
                    tray_app.apply_state(new_state);
                }

                while let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
                    if tray_app.is_quit_event(&event) {
                        tray_app.close();
                        cx.update(|cx| cx.quit());
                        return;
                    }
                }
            }
        })
        .detach();
    });

    Ok(())
}

fn init_tracing() -> error::AppResult<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .try_init()
        .map_err(|err| error::AppError::TracingInitError(err.to_string()))?;

    Ok(())
}
