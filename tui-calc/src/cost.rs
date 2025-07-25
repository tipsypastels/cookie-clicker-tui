pub fn building(building_base_cost: f64, count: u16) -> f64 {
    building_base_cost * 1.15f64.powi(count as _)
}

pub fn building_sell(cost: f64) -> f64 {
    cost * (1.0 / 4.5)
}
