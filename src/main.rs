#![allow(unused)]

mod api;
mod tray;

#[tokio::main]
async fn main() -> api::ApiResult<()> {
    let players = api::get_players().await?;
    dbg!(&players);

    tray::run_menu();

    Ok(())
}
