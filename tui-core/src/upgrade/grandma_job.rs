use super::effect_info::UpgradeEffectInfo;
use crate::{
    Building, Changeset, Cost, State,
    req::{Cmp, Req},
};

const COST_MULT: f64 = 50.0;

pub struct GrandmaJob {
    building: Building,
}

impl GrandmaJob {
    pub const fn new(building: Building) -> Self {
        debug_assert!(!building.is_cursor() && !building.is_grandma());
        Self { building }
    }

    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.building.base_cost() * COST_MULT)
    }

    pub fn req(&self) -> Req {
        Req::AllBox(Box::new([
            Req::BuildingCount(self.building, Cmp::AboveOrEq(15)),
            Req::BuildingCount(Building::Grandma, Cmp::AboveOrEq(1)),
        ]))
    }

    pub fn buy(&self, state: &mut State, changeset: &mut Changeset) {
        state
            .buildings
            .modify_has_grandma_job_upgrade(self.building, |b| *b = true, changeset);
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        let building = self.building;
        let num_req_for_1p = crate::calc::grandma_job_upgrade_num_req_for_1p(building);

        UpgradeEffectInfo::Grandma {
            building,
            num_req_for_1p,
        }
    }
}
