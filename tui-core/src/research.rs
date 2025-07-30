use crate::Changeset;
use cookie_clicker_tui_utils::refresh::Refresh;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const REFRESH: f64 = 5.0;

#[cfg(not(debug_assertions))]
const REFRESH: f64 = 30.0 * 60.0;

#[derive(Serialize, Deserialize, Debug)]
pub struct Research {
    completed: u8,
    refresh: Option<Refresh>,
}

impl Research {
    pub(crate) fn new() -> Self {
        Self {
            completed: 0,
            refresh: None,
        }
    }

    pub(crate) fn tick(&mut self, changeset: &mut Changeset) {
        if let Some(refresh) = self.refresh.as_mut()
            && refresh.finish()
        {
            self.completed = self.completed.saturating_add(1);
            self.refresh = None;

            changeset.available_upgrades = true;
            changeset.research_completed = true;
        }
    }

    pub(crate) fn start(&mut self) {
        self.refresh = Some(Refresh::new(REFRESH));
    }

    pub fn completed(&self) -> u8 {
        self.completed
    }
}
