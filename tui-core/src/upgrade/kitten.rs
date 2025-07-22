use super::effect_info::UpgradeEffectInfo;
use crate::{
    Cost, State,
    req::{Comparator, Req},
};

// in the base game, milk% can go above 100% to this value;
// i prefer to scale it to 100% being the max as this makes
// things like displaying it much simpler
const MAX_MILK_IN_BASE_GAME: f64 = 2488.0;

pub struct Kitten {
    milk_req: f64,
    mult: f64,
    cost: f64,
}

impl Kitten {
    pub const fn new(milk_req_in_base_game: f64, mult: f64, cost: f64) -> Self {
        let milk_req = milk_req_in_base_game / MAX_MILK_IN_BASE_GAME;
        Self {
            milk_req,
            mult,
            cost,
        }
    }

    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.cost)
    }

    pub fn req(&self) -> Req {
        Req::MilkRatio(Comparator::AboveOrEq(self.milk_req))
    }

    pub fn buy(&self, state: &mut State) {
        state.milk.add_kitten_mult(self.mult);
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        UpgradeEffectInfo::Kitten
    }
}
