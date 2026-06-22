use crate::{error::AppResult, state::AppEvent, state::AppState};
use std::path::Path;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
};
use tracing::error;
use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
};

pub fn install_menu_event_handler(proxy: EventLoopProxy<AppEvent>) {
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::Menu(event));
    }));
}

pub fn run(event_loop: EventLoop<AppEvent>, mut tray_app: TrayApp) -> ! {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                if let Err(err) = tray_app.initialize() {
                    error!(error = %err, "failed to initialize tray UI");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::UserEvent(AppEvent::Menu(event)) => {
                tray_app.handle_menu_event(event, control_flow)
            }
            Event::UserEvent(AppEvent::StateUpdated(state)) => tray_app.apply_state(state),
            _ => {}
        }
    })
}

pub struct TrayApp {
    menu: Menu,
    fave_players_submenu: Submenu,
    players_submenu: Submenu,
    weather_item: MenuItem,
    quit_item: MenuItem,
    tray_icon: Option<TrayIcon>,
    state: AppState,
}

impl TrayApp {
    pub fn new(initial_state: AppState) -> Self {
        let menu = Menu::new();
        let fave_players_submenu = Submenu::new(fave_players_submenu_title(&initial_state), true);
        let players_submenu = Submenu::new(players_submenu_title(&initial_state), true);
        let weather_item = MenuItem::new(weather_menu_text(&initial_state), false, None);
        let quit_item = MenuItem::new("Quit", true, None);

        let _ = menu.append_items(&[
            &fave_players_submenu,
            &players_submenu,
            &weather_item,
            &PredefinedMenuItem::separator(),
            &quit_item,
        ]);

        let mut tray_app = Self {
            menu,
            fave_players_submenu,
            players_submenu,
            weather_item,
            quit_item,
            tray_icon: None,
            state: initial_state,
        };

        tray_app.refresh_players_submenus();
        tray_app
    }

    fn initialize(&mut self) -> AppResult<()> {
        let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("icon.png");
        let icon = load_icon(&icon_path)?;

        let builder = TrayIconBuilder::new()
            .with_menu(Box::new(self.menu.clone()))
            .with_tooltip("Modern Beta")
            .with_icon(icon)
            .with_title(tray_title(&self.state));

        self.tray_icon = Some(builder.build()?);
        Ok(())
    }

    fn handle_menu_event(&mut self, event: MenuEvent, control_flow: &mut ControlFlow) {
        if event.id == self.quit_item.id() {
            self.tray_icon.take();
            *control_flow = ControlFlow::Exit;
        }
    }

    fn apply_state(&mut self, new_state: AppState) {
        self.state = new_state;
        self.players_submenu
            .set_text(players_submenu_title(&self.state));
        self.fave_players_submenu
            .set_text(fave_players_submenu_title(&self.state));
        self.weather_item.set_text(weather_menu_text(&self.state));
        self.refresh_players_submenus();

        if let Some(tray_icon) = self.tray_icon.as_ref() {
            tray_icon.set_title(Some(tray_title(&self.state)));
        }
    }

    fn clear_players_submenus(&mut self) {
        while self.players_submenu.remove_at(0).is_some() {}
        while self.fave_players_submenu.remove_at(0).is_some() {}
    }

    fn refresh_players_submenus(&mut self) {
        self.clear_players_submenus();

        if self.state.player_names.is_empty() {
            let placeholder = MenuItem::new(players_placeholder_text(&self.state), false, None);
            let _ = self.players_submenu.append(&placeholder);
            let _ = self.fave_players_submenu.append(&placeholder);
            return;
        }

        let mut player_names = self.state.player_names.clone();
        player_names.sort_unstable_by_key(|name| name.to_ascii_lowercase());

        for player_name in player_names {
            let item = MenuItem::new(&player_name, false, None);
            let _ = self.players_submenu.append(&item);

            if self.state.config.favorite_players.contains(&player_name) {
                let item = MenuItem::new(&player_name, false, None);
                let _ = self.fave_players_submenu.append(&item);
            }
        }
    }
}

fn players_submenu_title(state: &AppState) -> String {
    match state.player_count {
        Some(count) => format!("Online players ({count})"),
        None => "Online players".to_string(),
    }
}

fn fave_players_submenu_title(state: &AppState) -> String {
    let online_favorites = state
        .player_names
        .iter()
        .filter(|&name| state.config.favorite_players.contains(name))
        .count();

    match (state.config.favorite_players.is_empty(), online_favorites) {
        (true, _) => "Favorite players".to_string(),
        (false, count) => format!("Favorite players ({count})"),
    }
}

fn players_placeholder_text(state: &AppState) -> String {
    if state.player_count.is_none() {
        return "Loading...".to_string();
    }

    match state.player_count {
        Some(0) => "No players online".to_string(),
        Some(_) => "Names unavailable".to_string(),
        None => "Loading...".to_string(),
    }
}

fn weather_menu_text(state: &AppState) -> String {
    match &state.weather_text {
        Some(weather) => format!("Weather: {weather}"),
        None => "Weather: --".to_string(),
    }
}

fn tray_title(state: &AppState) -> String {
    match state.player_count {
        Some(count) => count.to_string(),
        None => "--".to_string(),
    }
}

fn load_icon(path: &Path) -> AppResult<tray_icon::Icon> {
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    tray_icon::Icon::from_rgba(rgba, width, height).map_err(|err| err.into())
}
