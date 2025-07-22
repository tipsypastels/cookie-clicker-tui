use super::effect_info::UpgradeEffectInfo;
use crate::{Building, State, req::Req};

const COST_MULT: f64 = 50.0;

pub struct Grandma {
    building: Building,
}

impl Grandma {
    pub const fn new(building: Building) -> Self {
        debug_assert!(!building.is_cursor() && !building.is_grandma());
        Self { building }
    }

    pub fn cost(&self) -> f64 {
        self.building.base_cost() * COST_MULT
    }

    pub fn req(&self) -> Req {
        Req::AllBox(Box::new([
            Req::BuildingCountMin(self.building, 15),
            Req::BuildingCountMin(Building::Grandma, 1),
        ]))
    }

    pub fn buy(&self, state: &mut State) {
        state
            .buildings
            .modify(self.building, |b| b.has_grandma_upgrade = true);
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        let building = self.building;
        let num_req_for_1p = crate::calc::grandma_upgrade_num_req_for_1p(building);

        UpgradeEffectInfo::Grandma {
            building,
            num_req_for_1p,
        }
    }
}
