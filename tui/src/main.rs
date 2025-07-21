mod app;
mod event;
mod storage;
mod ui;

use self::{app::App, storage::Storage};
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut storage = Storage::new(cli.dry_run)?;
    let core = storage.core().await?;

    let mut term = ratatui::init();
    let app = App::new(storage, core);
    let res = app.run(&mut term).await;

    ratatui::restore();
    res
}
