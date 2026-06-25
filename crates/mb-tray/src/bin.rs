#![allow(clippy::result_large_err)]

mod config;
mod error;
mod state;
mod tray;
mod ui;
mod window;
mod worker;

use std::{sync::mpsc, time::Duration};
use tracing::error;
use tracing_subscriber::EnvFilter;

fn main() -> error::AppResult<()> {
    init_tracing()?;

    let config = config::AppConfig::load()?;
    let shared_state = state::SharedAppState::new(state::AppState {
        config,
        ..Default::default()
    });
    let initial_state = shared_state.current();

    ui::run(move |app| {
        if let Err(err) = run_tray_app(app, initial_state, shared_state) {
            error!(error = %err, "failed to start tray app");
        }
    });

    Ok(())
}

fn run_tray_app(
    app: &mut gpui::App,
    initial_state: state::AppState,
    shared_state: state::SharedAppState,
) -> error::AppResult<()> {
    let mut tray_app = tray::TrayApp::try_new(initial_state)?;
    tray_app.initialize()?;

    let (tx, rx) = mpsc::channel::<state::AppState>();
    worker::Worker::new(shared_state, tx).spawn();

    let mut window_manager = window::WindowManager::default();

    app.spawn(async move |cx| {
        loop {
            cx.background_executor()
                .timer(Duration::from_millis(200))
                .await;

            while let Ok(new_state) = rx.try_recv() {
                tray_app.apply_state(new_state);
            }

            while let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
                if tray_app.is_lookup_event(&event) {
                    if let Err(err) = cx.update(|cx| window_manager.open_or_focus_profile(cx)) {
                        error!(error = %err, "failed to open or focus lookup window");
                    }
                    continue;
                }

                if tray_app.is_quit_event(&event) {
                    tray_app.close();
                    cx.update(|cx| cx.quit());
                    return;
                }
            }
        }
    })
    .detach();

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
