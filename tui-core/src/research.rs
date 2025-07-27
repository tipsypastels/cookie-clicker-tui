use crate::macros;
use cookie_clicker_tui_utils::refresh::Refresh;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const REFRESH: f64 = 5.0;

#[cfg(not(debug_assertions))]
const REFRESH: f64 = 30.0 * 60.0;

#[derive(Debug)]
pub struct Research {
    state: ResearchState,
    just_completed: bool,
}

impl Research {
    pub(crate) fn new() -> Self {
        Self::from_state(ResearchState {
            completed: 0,
            refresh: None,
        })
    }

    fn from_state(state: ResearchState) -> Self {
        Self {
            state,
            just_completed: false,
        }
    }

    pub(crate) fn tick(&mut self) {
        if let Some(refresh) = self.state.refresh.as_mut()
            && refresh.finish()
        {
            self.state.completed = self.state.completed.saturating_add(1);
            self.just_completed = true;
            self.state.refresh = None;
        } else if self.just_completed {
            self.just_completed = false;
        }
    }

    pub(crate) fn start(&mut self) {
        self.state.refresh = Some(Refresh::new(REFRESH));
    }

    pub fn completed(&self) -> u8 {
        self.state.completed
    }

    pub fn just_completed(&self) -> bool {
        self.just_completed
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ResearchState {
    completed: u8,
    refresh: Option<Refresh>,
}

macros::serialize_via_state!(Research => ResearchState as |r| r.state);
macros::deserialize_via_state!(Research => ResearchState as Research::from_state);
