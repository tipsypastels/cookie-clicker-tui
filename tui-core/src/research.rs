use crate::macros;
use cookie_clicker_tui_utils::frames::RefreshClock;

#[cfg(debug_assertions)]
const REFRESH: u32 = 5;

#[cfg(not(debug_assertions))]
const REFRESH: u32 = 30 * 60;

#[derive(Debug)]
pub struct Research {
    completed: u8,
    just_completed: bool,
    // TODO: Persist clocks.
    refresh: Option<RefreshClock<REFRESH>>,
}

impl Research {
    pub(crate) fn new() -> Self {
        Self::from_completed(0)
    }

    fn from_completed(completed: u8) -> Self {
        Self {
            completed,
            just_completed: false,
            refresh: None,
        }
    }

    pub(crate) fn tick(&mut self) {
        if let Some(refresh) = self.refresh.as_mut()
            && refresh.finish()
        {
            self.completed = self.completed.saturating_add(1);
            self.just_completed = true;
            self.refresh = None;
        } else if self.just_completed {
            self.just_completed = false;
        }
    }

    pub(crate) fn start(&mut self) {
        self.refresh = Some(RefreshClock::new());
    }

    pub fn completed(&self) -> u8 {
        self.completed
    }

    pub fn just_completed(&self) -> bool {
        self.just_completed
    }
}

macros::serialize_via_state!(Research => u8 as |r| r.completed);
macros::deserialize_via_state!(Research => u8 as Research::from_completed);
