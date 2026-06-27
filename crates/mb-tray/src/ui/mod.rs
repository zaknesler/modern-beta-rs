use gpui::{
    AnyWindowHandle, App, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px,
    size,
};
use gpui_component::Root;

pub mod client;
pub mod macos;
pub mod views;

pub fn run(setup: impl FnOnce(&mut App) + 'static) {
    gpui_platform::application().run(move |cx| {
        macos::configure_activation_policy();
        gpui_component::init(cx);
        setup(cx);
    });
}

pub fn open_profile_window(cx: &mut App) -> gpui::Result<AnyWindowHandle> {
    let bounds = Bounds::centered(None, size(px(480.), px(360.0)), cx);

    let handle = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Modern Beta".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let view = views::search::ProfileSearchView::view(window, cx);
            cx.new(|cx| Root::new(view, window, cx))
        },
    )?;

    cx.activate(true);
    handle.update(cx, |_, window, _| window.activate_window())?;

    Ok(handle.into())
}
