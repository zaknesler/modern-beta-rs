use crate::ui;
use gpui::{AnyWindowHandle, App};

#[derive(Default)]
struct ProfileLookupWindow {
    handle: Option<AnyWindowHandle>,
    username: Option<String>,
}

#[derive(Default)]
pub struct WindowManager {
    lookup_window: ProfileLookupWindow,
}

impl WindowManager {
    pub fn open_or_focus_profile(
        &mut self,
        cx: &mut App,
        username: Option<String>,
    ) -> gpui::Result<()> {
        // Reuse window if it's already open
        if let ProfileLookupWindow {
            handle: Some(handle),
            ..
        } = self.lookup_window
        {
            cx.activate(true);

            if handle
                .update(cx, |_, window, _| window.activate_window())
                .is_ok()
            {
                return Ok(());
            }
        }

        let handle = ui::open_profile_window(cx, username.clone())?;
        self.lookup_window = ProfileLookupWindow {
            handle: Some(handle),
            username,
        };

        Ok(())
    }
}
