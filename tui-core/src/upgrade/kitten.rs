use super::effect_info::UpgradeEffectInfo;
use crate::{
    Changeset, Cost, State,
    req::{Cmp, Req},
};

pub struct Kitten {
    achievement_req: usize,
    mult: f64,
    cost: f64,
}

impl Kitten {
    pub const fn new(achievement_req: usize, mult: f64, cost: f64) -> Self {
        Self {
            achievement_req,
            mult,
            cost,
        }
    }

    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.cost)
    }

    pub fn req(&self) -> Req {
        Req::AchievementCount(Cmp::AboveOrEq(self.achievement_req))
    }

    pub fn buy(&self, state: &mut State, changeset: &mut Changeset) {
        state.milk.add_kitten_factor(self.mult, changeset);
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        UpgradeEffectInfo::Kitten
    }
}
