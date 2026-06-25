use crate::ui;
use gpui::{AnyWindowHandle, App};

#[derive(Default)]
pub struct WindowManager {
    lookup_window: Option<AnyWindowHandle>,
}

impl WindowManager {
    pub fn open_or_focus_profile(&mut self, cx: &mut App) -> gpui::Result<()> {
        if let Some(handle) = self.lookup_window {
            cx.activate(true);

            if handle
                .update(cx, |_, window, _| window.activate_window())
                .is_ok()
            {
                return Ok(());
            }

            self.lookup_window = None;
        }

        let handle = ui::open_profile_window(cx)?;
        self.lookup_window = Some(handle);

        Ok(())
    }
}
