use crate::{
    State,
    building::{Building, BuildingState, Buildings},
};

pub fn cps(state: &State) -> f64 {
    state.buildings.infos().map(|i| i.cps()).sum()
}

// TODO: Test this.
#[allow(clippy::let_and_return)]
pub fn building_cps(buildings: &Buildings, building: Building, state: BuildingState) -> f64 {
    let cps = building.base_cps()
        * state.count as f64
        * 2.0f64.powi(state.simple_tiered_upgrade_count as i32);

    let cps = match building {
        Building::Cursor => cps,
        Building::Grandma => cps * 2.0f64.powi(buildings.grandma_co_tiered_upgrade_count() as i32),
        _ if !state.has_grandma_co_tiered_upgrade => cps,
        _ => {
            let num_req_for_1p_increase = building as u16 - 2;
            let grandma_count = buildings.get(Building::Grandma).count;

            if grandma_count > num_req_for_1p_increase {
                let ratio = grandma_count / num_req_for_1p_increase;
                let mult = ratio as f64 * 0.01;
                cps * mult
            } else {
                cps
            }
        }
    };

    cps
}
