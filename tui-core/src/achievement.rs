use crate::{Computed, State, req::LateReq};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use std::collections::{BTreeSet, VecDeque};

#[derive(Debug)]
pub struct Achievements {
    owned: BTreeSet<Achievement>,
    display_queue: VecDeque<Achievement>,
    refresh: RefreshClock<10>,
}

impl Achievements {
    pub fn new() -> Self {
        Self {
            owned: BTreeSet::new(),
            display_queue: VecDeque::new(),
            refresh: RefreshClock::new(),
        }
    }

    pub fn tick(&mut self, state: &State, computed: &Computed) {
        if self.refresh.finish() {
            self.refresh.restart();
            self.display_queue.pop_front();

            for achievement in Achievement::variants() {
                if self.owned.contains(&achievement) {
                    continue;
                }

                if achievement.req().as_late_req().check(state, computed) {
                    self.owned.insert(achievement);
                    self.display_queue.push_back(achievement);
                }
            }
        }
    }

    pub fn owned(&self) -> &BTreeSet<Achievement> {
        &self.owned
    }

    pub fn queued(&self) -> Option<Achievement> {
        self.display_queue.front().copied()
    }
}

#[derive(Assoc, Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
