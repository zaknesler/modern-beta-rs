use gpui::{App, ClickEvent, Context, Entity, Window, div, prelude::*, px};
use gpui_component::{
    button::*,
    input::{Input, InputState},
    *,
};

pub struct ProfileSearchView {
    input: Entity<InputState>,
}

impl ProfileSearchView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("")
                .placeholder("Enter username...")
        });

        Self { input }
    }

    fn on_click(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        let val = self.input.read(cx).value();

        println!("clicked: {val}")
    }
}

impl Render for ProfileSearchView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .on_click(cx.listener(Self::on_click)),
            )
    }
}
