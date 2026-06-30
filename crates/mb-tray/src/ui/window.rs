use crate::ui::views::lookup::ProfileLookupView;
use gpui::{
    AnyWindowHandle, App, Bounds, Entity, TitlebarOptions, WindowBounds, WindowOptions, prelude::*,
    px, size,
};
use gpui_component::Root;

#[derive(Default)]
struct ProfileLookupWindow {
    handle: Option<AnyWindowHandle>,
    view: Option<Entity<ProfileLookupView>>,
}

#[derive(Default)]
pub struct WindowManager {
    lookup_window: ProfileLookupWindow,
}

impl WindowManager {
    pub fn open_or_focus_lookup(
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

        let (handle, view) = open_lookup_window(cx, username)?;
        self.lookup_window = ProfileLookupWindow {
            handle: Some(handle),
            view: Some(view),
        };

        Ok(())
    }
}

pub fn open_lookup_window(
    cx: &mut App,
    username: Option<String>,
) -> gpui::Result<(AnyWindowHandle, Entity<ProfileLookupView>)> {
    let bounds = Bounds::centered(None, size(px(480.), px(360.0)), cx);

    let mut lookup_view = None;
    let handle = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Modern Beta".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let view = ProfileLookupView::view(window, cx, username);
            lookup_view = Some(view.clone());
            cx.new(|cx| Root::new(view, window, cx))
        },
    )?;

    cx.activate(true);
    handle.update(cx, |_, window, _| window.activate_window())?;

    Ok((handle.into(), lookup_view.unwrap()))
}
