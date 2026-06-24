use gpui::{
    Bounds, Context, Entity, TitlebarOptions, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, size,
};
use gpui_component::{
    button::*,
    input::{Input, InputState},
    *,
};

pub struct MainView {
    input: Entity<InputState>,
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .flex_row()
            .gap_4()
            .paddings(px(32.))
            .size_full()
            .child(Input::new(&self.input).cleanable(true).flex_1())
            .child(
                Button::new("ok")
                    .primary()
                    .label("Search")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);

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
                    let input = cx.new(|cx| {
                        InputState::new(window, cx)
                            .default_value("")
                            .placeholder("Enter username...")
                    });
                    let view = cx.new(|_| MainView { input });
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Failed to open window");
        })
        .detach();

        cx.activate(true);
    });
}
