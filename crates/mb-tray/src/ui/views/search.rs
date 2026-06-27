use crate::ui::client::ApiClient;
use gpui::{App, ClickEvent, Context, Entity, Task, Window, div, prelude::*, px};
use gpui_component::{
    StyledExt as _,
    button::*,
    input::{Input, InputState},
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
    task: Option<Task<()>>,
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

        Self {
            input,
            state: SearchState::Idle,
            task: None,
        }
    }

    fn on_search(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        let username = self.input.read(cx).value().trim().to_string();
        if username.is_empty() {
            return;
        }

        let client = ApiClient::from_app(cx);

        self.state = SearchState::Loading;
        cx.notify();

        let request =
            gpui_tokio::Tokio::spawn(
                cx,
                async move { client.get_player_profile(&username).await },
            );

        self.task = Some(cx.spawn(async move |this, cx| {
            let outcome = request.await;

            this.update(cx, |view, cx| {
                view.state = match outcome {
                    Ok(Ok(profile)) => SearchState::Loaded(profile),
                    Ok(Err(api_err)) => SearchState::Error(api_err.to_string()),
                    Err(join_err) => SearchState::Error(join_err.to_string()),
                };
                cx.notify();
            })
            .ok();
        }));
    }

    fn status(&self) -> String {
        match &self.state {
            SearchState::Idle => String::new(),
            SearchState::Loading => "Loading...".to_string(),
            SearchState::Error(err) => format!("Error: {err}"),
            SearchState::Loaded(profile) => {
                let name = profile.username.as_deref().unwrap_or("(unknown)");
                let online = if profile.online { "online" } else { "offline" };
                let rank = profile.rank_name.as_deref().unwrap_or("--");
                let hours = profile.played_time_seconds / 3600;
                format!("{name} — {online}\nrank: {rank}\nplayed: {hours}h")
            }
        }
    }
}

impl Render for ProfileSearchView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
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
                            .on_click(cx.listener(Self::on_search)),
                    ),
            )
            .child(div().whitespace_normal().child(self.status()))
    }
}
