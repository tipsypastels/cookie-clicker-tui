use super::effect_info::UpgradeEffectInfo;
use crate::{
    Building, Cost, State,
    req::{Cmp, Req},
};
use cookie_clicker_tui_utils::num;

pub struct Tiered {
    building: Building,
    building_req: u16,
    cost_mult: f64,
}

impl Tiered {
    pub const fn new(index: usize, building: Building) -> Self {
        debug_assert!(!building.is_cursor());

        let (building_req, cost_mult) = TEMPLATES[index];
        Self {
            building,
            building_req,
            cost_mult,
        }
    }

    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.building.base_cost() * self.cost_mult)
    }

    pub fn req(&self) -> Req {
        Req::BuildingCount(self.building, Cmp::AboveOrEq(self.building_req))
    }

    pub fn buy(&self, state: &mut State) {
        state
            .buildings
            .modify_tiered_upgrade_count(self.building, |c| *c += 1);
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        UpgradeEffectInfo::Tiered(self.building)
    }
}

const TEMPLATES: [(u16, f64); 15] = [
    (1, 10.0),
    (5, 50.0),
    (25, 500.0),
    (50, 50.0 * num::THOUSAND),
    (100, 5.0 * num::MILLION),
    (150, 500.0 * num::MILLION),
    (200, 500.0 * num::BILLION),
    (250, 500.0 * num::TRILLION),
    (300, 500.0 * num::QUADRILLION),
    (350, 500.0 * num::QUINTILLION),
    (400, 5.0 * num::SEPTILLION),
    (450, 50.0 * num::OCTILLION),
    (500, 500.0 * num::NONILLION),
    (550, 5.0 * num::UNDECILLION),
    (600, 50.0 * num::DUODECILLION),
];
