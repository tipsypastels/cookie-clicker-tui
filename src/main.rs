mod app;
mod event;
mod ui;

use self::app::App;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut term = ratatui::init();

    let app = App::new();
    let res = app.run(&mut term).await;

    ratatui::restore();
    res
}
