use anyhow::{Context, Result};
use cookie_clicker_tui_core::Core;
use cookie_clicker_tui_utils::frames::RefreshClock;
use std::path::{Path, PathBuf};
use tokio::fs;

const CORE_FILE_NAME: &str = "core.json";

#[derive(Debug)]
pub enum Storage {
    Dry,
    Persisted {
        core_path: Box<Path>,
        refresh: RefreshClock<10>,
    },
}

impl Storage {
    pub fn new(dry: bool) -> Result<Self> {
        if dry {
            return Ok(Self::Dry);
        }

        let dir = data_dir().context("could not find data directory")?;
        let core_path = dir.join(CORE_FILE_NAME).into();

        Ok(Self::Persisted {
            core_path,
            refresh: RefreshClock::new(),
        })
    }

    pub async fn core(&mut self) -> Result<Core> {
        match self {
            Self::Dry => Ok(Core::new()),
            Self::Persisted { core_path, .. } => {
                let s = fs::read_to_string(core_path)
                    .await
                    .context("could not read core file")?;

                match serde_json::from_str(&s) {
                    Ok(core) => Ok(core),
                    Err(_) => Ok(Core::new()),
                }
            }
        }
    }

    pub async fn tick(&mut self, core: &Core) -> Result<()> {
        let Self::Persisted { refresh, .. } = self else {
            return Ok(());
        };

        if refresh.finish() {
            refresh.restart();
            self.save(core).await
        } else {
            Ok(())
        }
    }

    pub async fn save(&mut self, core: &Core) -> Result<()> {
        let Self::Persisted { core_path, .. } = self else {
            return Ok(());
        };

        #[cfg(debug_assertions)]
        let res = serde_json::to_string_pretty(core);
        #[cfg(not(debug_assertions))]
        let res = serde_json::to_string(core);

        let json = res.context("could not serialize core")?;
        fs::write(core_path, json)
            .await
            .context("could not write core")
    }
}

#[cfg(debug_assertions)]
fn data_dir() -> Option<PathBuf> {
    std::env::current_dir().ok()
}

#[cfg(not(debug_assertions))]
fn data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|p| p.join("cookie-clicker-tui"))
}
