use crate::ui::{CloseWindow, client::ApiClient};
use gpui::{
    App, ClickEvent, Context, Entity, FocusHandle, Focusable, Window, div, prelude::*, px, rems,
};
use gpui_component::{
    ActiveTheme as _, Sizable, StyledExt as _,
    alert::Alert,
    button::*,
    input::{Input, InputState},
    spinner::Spinner,
};
use modern_beta_api::model::PlayerProfileResponse;

enum SearchState {
    Idle,
    Loading,
    Loaded(PlayerProfileResponse),
    Error(String),
}

pub struct ProfileSearchView {
    input: Entity<InputState>,
    state: SearchState,
    focus_handle: FocusHandle,
}

impl ProfileSearchView {
    pub fn view(window: &mut Window, cx: &mut App, username: Option<String>) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, username))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>, username: Option<String>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(username.clone().unwrap_or_default())
                .placeholder("Enter username...")
        });

        let focus_handle = input.read(cx).focus_handle(cx);
        window.focus(&focus_handle, cx);

        let mut view = Self {
            input,
            state: SearchState::Idle,
            focus_handle,
        };

        if username.is_some() {
            view.search(cx);
        }

        view
    }

    pub fn set_username(&mut self, username: String, window: &mut Window, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_value(username, window, cx));
        self.search(cx);
    }

    fn handle_submit(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.search(cx);
    }

    fn handle_close(&mut self, _: &CloseWindow, window: &mut Window, _: &mut Context<Self>) {
        window.remove_window();
    }

    fn search(&mut self, cx: &mut Context<Self>) {
        let username = self.input.read(cx).value().trim().to_string();
        if username.is_empty() {
            return;
        }

        let client = ApiClient::from_app(cx);

        self.state = SearchState::Loading;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let response =
                gpui_tokio::Tokio::spawn(
                    cx,
                    async move { client.get_player_profile(&username).await },
                )
                .await;

            this.update(cx, |view, cx| {
                view.state = match response {
                    Ok(Ok(profile)) => SearchState::Loaded(profile),
                    Ok(Err(api_err)) => SearchState::Error(api_err.to_string()),
                    Err(join_err) => SearchState::Error(join_err.to_string()),
                };
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn status(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.state {
            SearchState::Idle => div(),
            SearchState::Loading => div()
                .flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Spinner::new().large()),
            SearchState::Error(err) => {
                div().child(Alert::error("error-alert", format!("Error: {err}")))
            }
            SearchState::Loaded(profile) => {
                let online = if profile.online { "online" } else { "offline" };
                let hours = profile.played_time_seconds / 3600;
                let name = &profile.username;
                let rank = &profile.rank_name;
                let uuid = &profile.uuid;

                div()
                    .flex()
                    .v_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .p_4()
                            .border_1()
                            .border_color(cx.theme().sidebar_border)
                            .rounded_xl()
                            .bg(cx.theme().sidebar)
                            .items_center()
                            .overflow_hidden()
                            .child(
                                gpui::img(format!("https://mc-heads.net/avatar/{uuid}.png"))
                                    .size_16()
                                    .rounded_lg()
                                    .flex_shrink_0(),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_bold()
                                            .child(format!("{name}"))
                                            .line_height(rems(1.1)),
                                    )
                                    .child(format!("{rank}"))
                                    .child(format!("{online}")),
                            ),
                    )
                    .child(format!("played: {hours}h"))
            }
        }
    }
}

impl Render for ProfileSearchView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(cx.listener(Self::handle_close))
            .track_focus(&self.focus_handle)
            .v_flex()
            .gap_4()
            .paddings(px(32.))
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(Input::new(&self.input).cleanable(true).flex_1())
                    .child(
                        Button::new("ok")
                            .primary()
                            .label("Search")
                            .on_click(cx.listener(Self::handle_submit)),
                    ),
            )
            .child(self.status(cx))
    }
}
