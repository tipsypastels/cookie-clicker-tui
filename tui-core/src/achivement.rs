use crate::{Computed, State, req::LateReq};
use cookie_clicker_tui_utils::num;
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use std::collections::{BTreeSet, VecDeque};

const SECONDS_UNTIL_REFRESH: f64 = 10.0;

#[derive(Debug)]
pub struct Achivements {
    owned: BTreeSet<Achivement>,
    display_queue: VecDeque<Achivement>,
    ticks_until_refresh: u16,
}

impl Achivements {
    pub fn new(fps: f64) -> Self {
        Self {
            owned: BTreeSet::new(),
            display_queue: VecDeque::new(),
            ticks_until_refresh: (SECONDS_UNTIL_REFRESH * fps) as u16,
        }
    }

    pub fn tick(&mut self, fps: f64, state: &State, computed: &Computed) {
        if let Some(ticks_until_refresh) = self.ticks_until_refresh.checked_sub(1) {
            self.ticks_until_refresh = ticks_until_refresh;
        } else {
            self.ticks_until_refresh = (SECONDS_UNTIL_REFRESH * fps) as u16;
            self.display_queue.pop_front();

            for achivement in Achivement::variants() {
                if self.owned.contains(&achivement) {
                    continue;
                }

                if achivement.req().as_late_req().check(state, computed) {
                    self.owned.insert(achivement);
                    self.display_queue.push_back(achivement);
                }
            }
        }
    }

    pub fn owned(&self) -> &BTreeSet<Achivement> {
        &self.owned
    }

    pub fn queued(&self) -> Option<Achivement> {
        self.display_queue.front().copied()
    }
}

#[derive(Assoc, Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub fn req(&self) -> AchivementReq)]
#[name(base = "title case")]
pub enum Achivement {
    #[assoc(req = AchivementReq::CookiesBaked(1.0))]
    WakeAndBake,
    #[assoc(req = AchivementReq::CookiesBaked(1.0 * num::THOUSAND))]
    MakingSomeDough,
}

#[derive(Debug)]
pub enum AchivementReq {
    CookiesBaked(f64),
}

impl AchivementReq {
    pub(crate) fn as_late_req(&self) -> LateReq {
        match self {
            Self::CookiesBaked(v) => LateReq::CookiesAboveOrEq(*v),
        }
    }
}
