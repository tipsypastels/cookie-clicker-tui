use crate::Building;

pub enum UpgradeEffectInfo {
    Tiered(Building),
    Grandma {
        building: Building,
        num_req_for_1p: u16,
    },
    Kitten,
    Research {
        effect: UpgradeInfoEffectResearch,
        warning: Option<UpgradeInfoEffectResearchWarning>,
    },
}

pub enum UpgradeInfoEffectResearch {
    StartAndGrandmaCpsMult(f64),
    CpsMultiplierPercent(f64),
    GrandmaCpsDouble,
    GrandmaGainsCpsPerBuilding(Building, f64),
    ElderPledgesLastTwiceAsLong,
}

pub enum UpgradeInfoEffectResearchWarning {
    One,
    Two,
    Three,
}
