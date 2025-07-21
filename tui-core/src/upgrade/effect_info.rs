use crate::Building;

pub enum UpgradeEffectInfo {
    SimpleTiered(Building),
    GrandmaCoTiered {
        building: Building,
        num_req_for_1p: u16,
    },
}
