use crate::{State, building::Building};

pub fn cps(state: &State) -> f64 {
    state.buildings.infos().map(|i| i.cps()).sum()
}

pub fn building_cost(building: Building, count: u16) -> f64 {
    building.base_cost() * 1.15f64.powi(count as _)
}

pub fn building_sell_cost(cost: f64) -> f64 {
    cost * (1.0 / 4.5)
}

pub struct BuildingCps {
    pub building: Building,
    pub building_class: BuildingCpsClass,
    pub count: u16,
    pub simple_tiered_upgrade_count: u16,
}

pub enum BuildingCpsClass {
    Cursor,
    Grandma {
        grandma_co_tiered_upgrade_count: u16,
    },
    Other {
        grandma_count_for_co_tiered_upgrade: Option<u16>,
    },
}

#[allow(clippy::let_and_return)]
pub fn building_cps(
    BuildingCps {
        building,
        building_class,
        count,
        simple_tiered_upgrade_count,
    }: BuildingCps,
) -> f64 {
    let cps = building.base_cps() * count as f64 * 2.0f64.powi(simple_tiered_upgrade_count as i32);
    let cps = match building_class {
        BuildingCpsClass::Cursor => cps,
        BuildingCpsClass::Grandma {
            grandma_co_tiered_upgrade_count,
        } => cps * 2.0f64.powi(grandma_co_tiered_upgrade_count as i32),
        BuildingCpsClass::Other {
            grandma_count_for_co_tiered_upgrade: Some(grandma_count_for_co_tiered_upgrade),
        } => {
            let num_req_for_1p_increase = grandma_co_tiered_upgrade_num_req_for_1p(building);

            if grandma_count_for_co_tiered_upgrade > num_req_for_1p_increase {
                let ratio = grandma_count_for_co_tiered_upgrade / num_req_for_1p_increase;
                let addl = ratio as f64 * 0.01;
                cps + addl
            } else {
                cps
            }
        }
        BuildingCpsClass::Other {
            grandma_count_for_co_tiered_upgrade: None,
        } => cps,
    };

    cps
}

pub fn grandma_co_tiered_upgrade_num_req_for_1p(building: Building) -> u16 {
    building as u16 - 1
}
