use crate::state::AppState;

pub enum AppEvent {
    Menu(tray_icon::menu::MenuEvent),
    StateUpdated(AppState),
}
