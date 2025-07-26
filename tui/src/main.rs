mod app;
mod audio;
mod event;
mod save;
mod ui;

use self::{
    app::App,
    audio::Audio,
    save::{Save, SaveData, SaveOptions},
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
    free_everything: bool,

    /// Give a free "You" building to start with
    #[clap(long, requires = "dry_run")]
    free_you: bool,

    /// Greatly speeds up golden cookie spawning
    #[clap(long, requires = "dry_run")]
    fast_golden_cookies: bool,
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
    let SaveData {
        bakery_name,
        mut core,
    } = save.data().await?;

    if cli.free_everything {
        core.cheat_make_everything_free();
    }

    if cli.free_you {
        core.give_building(Building::You);
    }

    if cli.fast_golden_cookies {
        core.cheat_spawn_golden_cookies_fast();
    }

    let mut term = ratatui::init();
    let audio = Audio::new()?;

    let app = App::new(save, core, bakery_name, audio);
    let res = app.run(&mut term).await;

    ratatui::restore();
    res
}
