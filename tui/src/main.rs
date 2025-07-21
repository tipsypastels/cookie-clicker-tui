mod app;
mod event;
mod storage;
mod ui;

use self::{app::App, storage::Storage};
use anyhow::Result;
use clap::Parser;
use cookie_clicker_tui_core::Building;

#[derive(Parser)]
struct Cli {
    /// Start without reading or writing to the filesystem
    #[clap(long)]
    dry_run: bool,

    /// Give a free "You" building to start with
    #[clap(long, requires = "dry_run")]
    free_you: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut storage = Storage::new(cli.dry_run)?;
    let mut core = storage.core().await?;

    if cli.free_you {
        core.give_free_building(Building::You);
    }

    let mut term = ratatui::init();
    let app = App::new(storage, core);
    let res = app.run(&mut term).await;

    ratatui::restore();
    res
}
