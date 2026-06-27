pub struct ApiClient(pub modern_beta_api::Client);

impl gpui::Global for ApiClient {}

impl ApiClient {
    pub fn from_app(cx: &mut gpui::App) -> modern_beta_api::Client {
        cx.global::<Self>().0.clone()
    }
}
