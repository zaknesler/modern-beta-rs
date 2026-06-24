#[cfg(target_os = "macos")]
/// Ensure app does not appear in dock
pub fn configure_activation_policy() {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApp, NSApplicationActivationPolicy};

    let mtm = MainThreadMarker::new().expect("must be on the main thread");
    let app = NSApp(mtm);
    let _ = app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
}

#[cfg(not(target_os = "macos"))]
pub fn configure_activation_policy() {}
