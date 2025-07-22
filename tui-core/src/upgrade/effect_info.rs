use crate::Building;

pub enum UpgradeEffectInfo {
    Tiered(Building),
    Grandma {
        building: Building,
        num_req_for_1p: u16,
    },
    Kitten,
}
