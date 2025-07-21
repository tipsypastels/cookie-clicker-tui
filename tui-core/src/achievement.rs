use crate::{Computed, State, req::LateReq};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeSet, VecDeque};

// can't be a method since we need to access &state
// but also &mut Achievements, which is a field of state
pub fn tick(state: &mut State, computed: &Computed) {
    if state.achievements.refresh.finish() {
        state.achievements.refresh.restart();
        state.achievements.display_queue.pop_front();

        for achievement in Achievement::variants() {
            if state.achievements.owned.contains(&achievement) {
                continue;
            }

            if achievement.req().as_late_req().check(state, computed) {
                state.achievements.owned.insert(achievement);
                state.achievements.display_queue.push_back(achievement);
            }
        }
    }
}

#[derive(Debug)]
pub struct Achievements {
    owned: BTreeSet<Achievement>,
    display_queue: VecDeque<Achievement>,
    refresh: RefreshClock<10>,
}

impl Achievements {
    pub fn new() -> Self {
        Self::from_owned(BTreeSet::new())
    }

    fn from_owned(owned: BTreeSet<Achievement>) -> Self {
        Self {
            owned,
            display_queue: VecDeque::new(),
            refresh: RefreshClock::new(),
        }
    }

    pub fn owned(&self) -> &BTreeSet<Achievement> {
        &self.owned
    }

    pub fn queued(&self) -> Option<Achievement> {
        self.display_queue.front().copied()
    }
}

impl Serialize for Achievements {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.owned.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Achievements {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        BTreeSet::deserialize(de).map(Self::from_owned)
    }
}

#[derive(
    Assoc,
    Name,
    Variants,
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[func(pub fn req(&self) -> AchievementReq)]
#[name(base = "title case")]
pub enum Achievement {
    #[assoc(req = AchievementReq::CookiesBaked(1.0))]
    WakeAndBake,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::THOUSAND))]
    MakingSomeDough,
}

#[derive(Debug)]
pub enum AchievementReq {
    CookiesBaked(f64),
}

impl AchievementReq {
    fn as_late_req(&self) -> LateReq {
        match self {
            Self::CookiesBaked(v) => LateReq::CookiesAllTimeAboveOrEq(*v),
        }
    }
}
