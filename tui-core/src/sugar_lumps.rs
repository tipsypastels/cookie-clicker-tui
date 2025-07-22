use crate::{
    State,
    req::{Comparator, Req},
};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const UNLOCK_REQ: Req = Req::Cookies(Comparator::AboveOrEq(1.0 * num::BILLION));

pub fn tick(state: &mut State) {
    if state.sugar_lumps.state.just_unlocked {
        state.sugar_lumps.state.just_unlocked = false;
    }

    if !state.sugar_lumps.state.unlocked && state.sugar_lumps.unlock_refresh.finish() {
        if UNLOCK_REQ.check(state) {
            state.sugar_lumps.state.unlocked = true;
            state.sugar_lumps.state.just_unlocked = true;
        } else {
            state.sugar_lumps.unlock_refresh.restart();
        }
    }
}

#[derive(Debug)]
pub struct SugarLumps {
    state: SugarLumpState,
    unlock_refresh: RefreshClock<10>,
    grow_refresh: RefreshClock<{ 4 * 60 * 60 }>,
}

impl SugarLumps {
    pub(super) fn new() -> Self {
        Self {
            state: SugarLumpState::default(),
            unlock_refresh: RefreshClock::new(),
            grow_refresh: RefreshClock::new(),
        }
    }

    pub fn unlocked(&self) -> bool {
        self.state.unlocked
    }

    pub fn just_unlocked(&self) -> bool {
        self.state.just_unlocked
    }
}

impl Serialize for SugarLumps {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.state.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for SugarLumps {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self {
            state: SugarLumpState::deserialize(de)?,
            unlock_refresh: RefreshClock::new(),
            grow_refresh: RefreshClock::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SugarLumpState {
    count: u16,
    unlocked: bool,
    just_unlocked: bool,
}
