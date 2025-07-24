mod app;
mod event;
mod save;
mod ui;

use self::{
    app::App,
    save::{Save, SaveOptions},
};
use anyhow::Result;
use clap::Parser;
use cookie_clicker_tui_core::Building;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    /// The path to the save file
    path: Option<Box<Path>>,

    /// Start without reading or writing to the filesystem
    #[clap(long)]
    dry_run: bool,

    /// Show notifications on autosave
    #[clap(long, conflicts_with = "dry_run")]
    notify_on_autosave: bool,

    /// Make all purchases free
    #[clap(long, requires = "dry_run")]
    everything_free: bool,

    /// Give a free "You" building to start with
    #[clap(long, requires = "dry_run")]
    free_you: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let save_options = if cli.dry_run {
        SaveOptions::Dry
    } else {
        SaveOptions::Persisted {
            path: cli.path,
            notify_on_autosave: cli.notify_on_autosave,
        }
    };

    let mut save = Save::new(save_options)?;
    let mut core = save.data().await?.core;

    if cli.everything_free {
        core.make_everything_free();
    }

    if cli.free_you {
        core.give_building(Building::You);
    }

    let mut term = ratatui::init();
    let app = App::new(save, core);
    let res = app.run(&mut term).await;

    ratatui::restore();
    res
}
