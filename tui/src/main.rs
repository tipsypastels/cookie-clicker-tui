mod app;
mod event;
mod ui;

use self::app::App;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut term = ratatui::init();
    let app = App::new();
    let res = app.run(&mut term).await;

    ratatui::restore();
    res
}
