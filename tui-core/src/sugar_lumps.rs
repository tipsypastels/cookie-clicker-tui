use crate::{
    State, macros,
    req::{Cmp, Req},
};
use cookie_clicker_tui_utils::{num, refresh::Refresh};
use serde::{Deserialize, Serialize};

const REFRESH_LOCKED: f64 = 10.0;
const REFRESH_GROW: f64 = 10.0;
const UNLOCK_REQ: Req = Req::CookiesAllTime(Cmp::AboveOrEq(1.0 * num::BILLION));

pub fn tick(state: &mut State) {
    match &mut state.sugar_lumps.0 {
        SugarLumpsState::Locked { refresh } => {
            if refresh.finish() {
                if UNLOCK_REQ.check(state) {
                    state.sugar_lumps.0 = SugarLumpsState::Unlocked {
                        count: 0,
                        unlocked_just_now: true,
                        refresh: Refresh::new(REFRESH_GROW),
                    };
                    // borrow again to prevent errors
                } else if let SugarLumpsState::Locked { refresh } = &mut state.sugar_lumps.0 {
                    refresh.reset()
                }
            }
        }
        SugarLumpsState::Unlocked {
            count,
            unlocked_just_now,
            refresh,
        } => {
            *unlocked_just_now = false;

            if refresh.finish()
                && let Some(next_count) = count.checked_add(1)
            {
                *count = next_count;
                refresh.reset();
            }
        }
    }
}

#[derive(Debug)]
pub struct SugarLumps(SugarLumpsState);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "state")]
enum SugarLumpsState {
    Locked {
        refresh: Refresh,
    },
    Unlocked {
        count: u16,
        #[serde(default, skip_serializing)]
        unlocked_just_now: bool,
        refresh: Refresh,
    },
}

impl SugarLumps {
    pub(super) fn new() -> Self {
        Self(SugarLumpsState::Locked {
            refresh: Refresh::new(REFRESH_LOCKED),
        })
    }

    pub fn count(&self) -> u16 {
        match &self.0 {
            SugarLumpsState::Locked { .. } => 0,
            SugarLumpsState::Unlocked { count, .. } => *count,
        }
    }

    pub fn unlocked(&self) -> bool {
        matches!(self.0, SugarLumpsState::Unlocked { .. })
    }

    pub fn just_unlocked(&self) -> bool {
        match &self.0 {
            SugarLumpsState::Locked { .. } => false,
            SugarLumpsState::Unlocked {
                unlocked_just_now, ..
            } => *unlocked_just_now,
        }
    }
}

macros::serialize_via_state!(SugarLumps => SugarLumpsState as |s| s.0);
macros::deserialize_via_state!(SugarLumps => SugarLumpsState as SugarLumps);
