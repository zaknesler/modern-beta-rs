use gpui::{Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size};
use gpui_component::Root;

mod views;

pub fn run(setup: impl FnOnce(&mut gpui::App) + 'static) {
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);
        setup(cx);

        let bounds = Bounds::centered(None, size(px(480.), px(360.0)), cx);

        cx.spawn(async move |cx| {
            cx.open_window(
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
            )
            .expect("Failed to open window");
        })
        .detach();

        cx.activate(true);
    });
}
