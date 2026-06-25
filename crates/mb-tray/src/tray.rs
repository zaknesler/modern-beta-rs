use crate::{
    error::{AppError, AppResult},
    state::{AppState, OnlinePlayersState},
};
use image::EncodableLayout;
use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu, accelerator::Accelerator},
};

#[derive(rust_embed::Embed)]
#[folder = "assets"]
struct AssetDir;

pub struct TrayApp {
    menu: Menu,
    fave_players_submenu: Submenu,
    players_submenu: Submenu,
    weather_item: MenuItem,
    lookup_item: MenuItem,
    quit_item: MenuItem,
    tray_icon: Option<TrayIcon>,
    state: AppState,
}

impl TrayApp {
    pub fn try_new(initial_state: AppState) -> AppResult<Self> {
        let menu = Menu::new();
        let fave_players_submenu = Submenu::new(fave_players_submenu_title(&initial_state), true);
        let players_submenu = Submenu::new(players_submenu_title(&initial_state), true);
        let weather_item = MenuItem::new(weather_menu_text(&initial_state), false, None);
        let lookup_item = MenuItem::new(
            "Lookup...",
            true,
            Some(Accelerator::new(
                None,
                tray_icon::menu::accelerator::Code::KeyL,
            )),
        );
        let quit_item = MenuItem::new("Quit", true, None);

        menu.append_items(&[
            &fave_players_submenu,
            &players_submenu,
            &weather_item,
            &PredefinedMenuItem::separator(),
            &lookup_item,
            &PredefinedMenuItem::separator(),
            &quit_item,
        ])?;

        let mut tray_app = Self {
            menu,
            fave_players_submenu,
            players_submenu,
            weather_item,
            lookup_item,
            quit_item,
            tray_icon: None,
            state: initial_state,
        };

        tray_app.refresh_players_submenus();

        Ok(tray_app)
    }

    pub fn initialize(&mut self) -> AppResult<()> {
        let icon = load_icon()?;

        self.tray_icon = Some(
            TrayIconBuilder::new()
                .with_menu(Box::new(self.menu.clone()))
                .with_tooltip("Modern Beta")
                .with_icon(icon)
                .with_title(tray_title(&self.state))
                .build()?,
        );

        Ok(())
    }

    pub fn is_lookup_event(&self, event: &MenuEvent) -> bool {
        event.id == self.lookup_item.id()
    }

    pub fn is_quit_event(&self, event: &MenuEvent) -> bool {
        event.id == self.quit_item.id()
    }

    pub fn close(&mut self) {
        self.tray_icon.take();
    }

    pub fn apply_state(&mut self, new_state: AppState) {
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

        let mut player_names = match self.state.online_players() {
            OnlinePlayersState::Loaded(names) => names,
            _ => {
                self.append_players_placeholder();
                return;
            }
        };

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

    fn append_players_placeholder(&mut self) {
        let placeholder = MenuItem::new(players_placeholder_text(&self.state), false, None);
        let _ = self.players_submenu.append(&placeholder);
        let _ = self.fave_players_submenu.append(&placeholder);
    }
}

fn players_submenu_title(state: &AppState) -> String {
    match state.online_players_count() {
        Some(count) => format!("Online players ({count})"),
        None => "Online players".to_string(),
    }
}

fn fave_players_submenu_title(state: &AppState) -> String {
    match state.online_favorite_players_count() {
        Some(count) => format!("Favorite players ({count})"),
        None => "Favorite players".to_string(),
    }
}

fn players_placeholder_text(state: &AppState) -> &'static str {
    match state.online_players() {
        OnlinePlayersState::Loading => "Loading...",
        OnlinePlayersState::Empty => "No players online",
        OnlinePlayersState::Loaded(_) => "Players available",
    }
}

fn weather_menu_text(state: &AppState) -> String {
    match state.world() {
        Some(world) => format!("Weather: {}", world.weather_state()),
        None => "Weather: --".to_string(),
    }
}

fn tray_title(state: &AppState) -> String {
    match (
        state.online_players_count(),
        state.online_favorite_players_count(),
    ) {
        (Some(count), Some(fave_count)) => format!("{count} ({fave_count})"),
        (Some(count), None) => format!("{count}"),
        _ => "--".to_string(),
    }
}

fn load_icon() -> AppResult<tray_icon::Icon> {
    let file = AssetDir::get("icon.png").ok_or_else(|| AppError::IconNotFound)?;
    let image = image::load_from_memory(file.data.as_bytes())?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    tray_icon::Icon::from_rgba(rgba, width, height).map_err(|err| err.into())
}
