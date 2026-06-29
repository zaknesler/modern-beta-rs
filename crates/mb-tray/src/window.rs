use crate::ui;
use crate::ui::views::search::ProfileSearchView;
use gpui::{AnyWindowHandle, App, Entity};

#[derive(Default)]
struct ProfileLookupWindow {
    handle: Option<AnyWindowHandle>,
    view: Option<Entity<ProfileSearchView>>,
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
            view: Some(ref view),
        } = self.lookup_window
        {
            if handle
                .update(cx, |_, window, cx| {
                    if let Some(username) = username.clone() {
                        view.update(cx, |v, cx| v.set_username(username, window, cx));
                    }
                    window.activate_window();
                })
                .is_ok()
            {
                cx.activate(true);
                return Ok(());
            }
        }

        let (handle, view) = ui::open_profile_window(cx, username)?;
        self.lookup_window = ProfileLookupWindow {
            handle: Some(handle),
            view: Some(view),
        };

        Ok(())
    }
}
