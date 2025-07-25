use anyhow::{Context, Result};
use cookie_clicker_tui_core::Core;
use cookie_clicker_tui_utils::frames::RefreshClock;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Deserialize, Default)]
pub struct SaveData {
    #[serde(default)]
    pub bakery_name: Option<Box<str>>,
    pub core: Core,
}

#[derive(Serialize)]
struct SaveDataRef<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    bakery_name: Option<&'a str>,
    core: &'a Core,
}

pub enum SaveOptions {
    Dry,
    Persisted {
        path: Option<Box<Path>>,
        notify_on_autosave: bool,
    },
}

#[derive(Debug)]
pub struct Save(Inner);

#[derive(Debug)]
enum Inner {
    Dry,
    PersistedDefaultPath(PersistedShared, bool),
    PersistedManualPath(PersistedShared),
}

#[derive(Debug)]
struct PersistedShared {
    path: Box<Path>,
    refresh: RefreshClock<10>,
    notify_on_autosave: bool,
    notify_just_saved: bool,
}

impl Save {
    pub fn new(options: SaveOptions) -> Result<Self> {
        match options {
            SaveOptions::Dry => Ok(Self(Inner::Dry)),
            SaveOptions::Persisted {
                path,
                notify_on_autosave,
            } => {
                let (path, path_manual) = match path {
                    Some(path) => (path, true),
                    None => (default_path()?, false),
                };

                let shared = PersistedShared {
                    path,
                    refresh: RefreshClock::new(),
                    notify_on_autosave,
                    notify_just_saved: false,
                };

                if path_manual {
                    Ok(Self(Inner::PersistedManualPath(shared)))
                } else {
                    Ok(Self(Inner::PersistedDefaultPath(shared, false)))
                }
            }
        }
    }

    pub fn notify_just_saved(&self) -> bool {
        self.shared()
            .map(|s| s.notify_just_saved)
            .unwrap_or_default()
    }

    pub fn notify_swallowed_parse_error(&self) -> bool {
        match &self.0 {
            Inner::PersistedDefaultPath(_, e) => *e,
            _ => false,
        }
    }

    pub async fn data(&mut self) -> Result<SaveData> {
        struct Context;
        struct Swallow<'a>(&'a mut bool);

        trait OpenHandler {
            type Output;

            fn open_file_fallback() -> Self::Output;

            fn handle_parse_result(
                &mut self,
                path: &Path,
                result: serde_json::Result<SaveData>,
            ) -> Self::Output;
        }

        impl OpenHandler for Context {
            type Output = Result<SaveData>;

            fn open_file_fallback() -> Self::Output {
                Ok(SaveData::default())
            }

            fn handle_parse_result(
                &mut self,
                path: &Path,
                result: serde_json::Result<SaveData>,
            ) -> Self::Output {
                result.with_context(|| format!("failed to parse save file '{path:?}'"))
            }
        }

        impl OpenHandler for Swallow<'_> {
            type Output = SaveData;

            fn open_file_fallback() -> Self::Output {
                SaveData::default()
            }

            fn handle_parse_result(
                &mut self,
                _path: &Path,
                result: serde_json::Result<SaveData>,
            ) -> Self::Output {
                match result {
                    Ok(data) => data,
                    Err(_) => {
                        *self.0 = true;
                        SaveData::default()
                    }
                }
            }
        }

        async fn open<H: OpenHandler>(path: &Path, mut handler: H) -> H::Output {
            let text = match fs::read_to_string(path).await {
                Ok(text) => text,
                // TODO: Consider only doing this for nonexistant files,
                // not all fs errors.
                Err(_) => return H::open_file_fallback(),
            };
            handler.handle_parse_result(path, serde_json::from_str(&text))
        }

        match &mut self.0 {
            Inner::Dry => Ok(SaveData::default()),
            Inner::PersistedDefaultPath(shared, error) => {
                Ok(open(&shared.path, Swallow(error)).await)
            }
            Inner::PersistedManualPath(shared) => open(&shared.path, Context).await,
        }
    }

    pub async fn tick(&mut self, core: &Core, bakery_name: Option<&str>) -> Result<()> {
        let Some(shared) = self.shared_mut() else {
            return Ok(());
        };

        shared.notify_just_saved = false;

        if shared.refresh.finish() {
            shared.refresh.restart();
            self._save(core, bakery_name, true).await
        } else {
            Ok(())
        }
    }

    pub async fn save(&mut self, core: &Core, bakery_name: Option<&str>) -> Result<()> {
        self._save(core, bakery_name, false).await
    }

    async fn _save(&mut self, core: &Core, bakery_name: Option<&str>, auto: bool) -> Result<()> {
        if self.notify_swallowed_parse_error() {
            return Ok(());
        }

        let Some(shared) = self.shared_mut() else {
            return Ok(());
        };

        let data = SaveDataRef { bakery_name, core };
        #[cfg(debug_assertions)]
        let res = serde_json::to_string_pretty(&data);
        #[cfg(not(debug_assertions))]
        let res = serde_json::to_string(&data);

        let json = res.context("could not serialize save data")?;
        fs::write(&shared.path, json)
            .await
            .context("could not write save data")?;

        if !auto || shared.notify_on_autosave {
            shared.notify_just_saved = true;
        }

        Ok(())
    }

    fn shared(&self) -> Option<&PersistedShared> {
        match &self.0 {
            Inner::Dry => None,
            Inner::PersistedDefaultPath(shared, _) => Some(shared),
            Inner::PersistedManualPath(shared) => Some(shared),
        }
    }

    fn shared_mut(&mut self) -> Option<&mut PersistedShared> {
        match &mut self.0 {
            Inner::Dry => None,
            Inner::PersistedDefaultPath(shared, _) => Some(shared),
            Inner::PersistedManualPath(shared) => Some(shared),
        }
    }
}

#[cfg(debug_assertions)]
fn default_path() -> Result<Box<Path>> {
    Ok(std::env::current_dir()
        .context("could not determine current directory")?
        .join("save.json")
        .into())
}

#[cfg(not(debug_assertions))]
fn default_path() -> Result<Box<Path>> {
    Ok(dirs::data_dir()
        .context("could not determine data directory")?
        .join("cookie-clicker-tui")
        .join("save.json")
        .into())
}
