use crate::{
    State,
    req::{Comparator, Req},
};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const UNLOCK_REQ: Req = Req::Cookies(Comparator::AboveOrEq(1.0 * num::BILLION));

pub fn tick(state: &mut State) {
    match &mut state.sugar_lumps.0 {
        Inner::Locked { refresh } => {
            if refresh.finish() {
                if UNLOCK_REQ.check(state) {
                    state.sugar_lumps.0 = Inner::Unlocked {
                        count: 0,
                        unlocked_just_now: true,
                        refresh: RefreshClock::new(),
                    };
                    // borrow again to prevent errors
                } else if let Inner::Locked { refresh } = &mut state.sugar_lumps.0 {
                    refresh.restart()
                }
            }
        }
        Inner::Unlocked {
            count,
            unlocked_just_now,
            refresh,
        } => {
            *unlocked_just_now = false;

            if refresh.finish()
                && let Some(next_count) = count.checked_add(1)
            {
                *count = next_count;
                refresh.restart();
            }
        }
    }
}

#[derive(Debug)]
pub struct SugarLumps(Inner);

#[derive(Debug)]
enum Inner {
    Locked {
        refresh: RefreshClock<10>,
    },
    Unlocked {
        count: u16,
        unlocked_just_now: bool,
        refresh: RefreshClock<{ 60 * 60 }>,
    },
}

impl SugarLumps {
    pub(super) fn new() -> Self {
        Self(Inner::Locked {
            refresh: RefreshClock::new(),
        })
    }

    fn from_repr(repr: Repr) -> Self {
        Self(match repr {
            Repr::Locked => Inner::Locked {
                refresh: RefreshClock::new(),
            },
            Repr::Unlocked { count } => Inner::Unlocked {
                count,
                unlocked_just_now: false,
                refresh: RefreshClock::new(),
            },
        })
    }

    fn as_repr(&self) -> Repr {
        match &self.0 {
            Inner::Locked { .. } => Repr::Locked,
            Inner::Unlocked { count, .. } => Repr::Unlocked { count: *count },
        }
    }

    pub fn count(&self) -> u16 {
        match &self.0 {
            Inner::Locked { .. } => 0,
            Inner::Unlocked { count, .. } => *count,
        }
    }

    pub fn unlocked(&self) -> bool {
        matches!(self.0, Inner::Unlocked { .. })
    }

    pub fn just_unlocked(&self) -> bool {
        match &self.0 {
            Inner::Locked { .. } => false,
            Inner::Unlocked {
                unlocked_just_now, ..
            } => *unlocked_just_now,
        }
    }
}

impl Serialize for SugarLumps {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.as_repr().serialize(ser)
    }
}

impl<'de> Deserialize<'de> for SugarLumps {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Repr::deserialize(de).map(Self::from_repr)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "state")]
enum Repr {
    Locked,
    Unlocked { count: u16 },
}

impl From<&Inner> for Repr {
    fn from(inner: &Inner) -> Self {
        match inner {
            Inner::Locked { .. } => Self::Locked,
            Inner::Unlocked { count, .. } => Self::Unlocked { count: *count },
        }
    }
}
