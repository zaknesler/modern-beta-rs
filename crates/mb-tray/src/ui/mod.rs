use gpui::{App, KeyBinding, QuitMode, actions};

pub mod client;
pub mod macos;
pub mod views;
pub mod window;

actions!(app, [CloseWindow]);

pub fn run(setup: impl FnOnce(&mut App) + 'static) {
    let http_client = std::sync::Arc::new(reqwest_client::ReqwestClient::new());

    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .with_http_client(http_client)
        .run(move |cx| {
            cx.bind_keys([
                KeyBinding::new("ctrl-w", CloseWindow, None),
                KeyBinding::new("cmd-w", CloseWindow, None),
            ]);

            macos::configure_activation_policy();
            cx.set_quit_mode(QuitMode::Explicit);
            gpui_component::init(cx);
            setup(cx);
        });
}
