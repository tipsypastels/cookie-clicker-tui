use crate::{State, building::Building};
use cookie_clicker_tui_calc as calc;

pub use calc::cps::building::CpsClass as BuildingCpsClass;

pub fn cps(state: &State) -> f64 {
    let base = calc::cps::base::Cps {
        building_cpses: state.buildings.infos().map(|i| i.cps()),
    };
    let addl = calc::cps::addl::Cps {
        grandmapocalypse_mults: state.grandmapocalypse.cps_mults().iter().copied(),
        kitten_mult: state.milk.cps_mult(),
    };
    calc::cps::Cps::new(base, addl).total
}

pub fn building_cps<AddlCpsPerOwnedBuildingCounts>(
    building: Building,
    building_class: BuildingCpsClass,
    count: u16,
    tiered_upgrade_count: u16,
    addl_cps_per_owned_building_counts: AddlCpsPerOwnedBuildingCounts,
) -> f64
where
    AddlCpsPerOwnedBuildingCounts: Iterator<Item = (u16, f64)>,
{
    calc::cps::building::Cps {
        building_no: building as u16,
        building_base_cps: building.base_cps(),
        building_class,
        count,
        tiered_upgrade_count,
        addl_cps_per_owned_building_counts,
    }
    .calc()
}

pub fn kitten_cps_mult(milk_percentage: u16, kitten_factors: &[f64]) -> f64 {
    calc::cps::kittens::Cps {
        milk_percentage,
        kitten_factors: kitten_factors.iter().copied(),
    }
    .calc()
}

pub fn building_cost(building: Building, count: u16) -> f64 {
    calc::cost::building(building.base_cost(), count)
}

pub fn building_sell_cost(cost: f64) -> f64 {
    calc::cost::building_sell(cost)
}

pub fn grandma_job_upgrade_num_req_for_1p(building: Building) -> u16 {
    calc::upgrade::grandma_job_num_req_for_1p(building as u16)
}
