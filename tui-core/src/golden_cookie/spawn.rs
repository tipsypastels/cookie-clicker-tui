use crate::spawner::Spawner;
use cookie_clicker_tui_utils::refresh::Refresh;
use serde::{Deserialize, Serialize};

const DEFAULT_TMIN_SECS: f64 = 300.0;
const DEFAULT_TMAX_SECS: f64 = 900.0;

#[derive(Serialize, Deserialize, Debug)]
pub enum SpawningMode {
    Normal { spawner: Spawner },
    Chain { refresh: Refresh },
}

impl SpawningMode {
    pub fn tick(&mut self) {}
}

impl Default for SpawningMode {
    fn default() -> Self {
        Self::Normal {
            spawner: Spawner::new(DEFAULT_TMIN_SECS, DEFAULT_TMAX_SECS),
        }
    }
}
