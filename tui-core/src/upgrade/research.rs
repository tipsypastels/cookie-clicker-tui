use super::effect_info::{
    UpgradeEffectInfo, UpgradeInfoEffectResearch, UpgradeInfoEffectResearchWarning,
};
use crate::{
    Achievement, Building, Cost, GrandmapocalypsePhase, State,
    req::{Cmp, Req},
};
use cookie_clicker_tui_utils::num;
use enum_assoc::Assoc;

#[derive(Assoc, Copy, Clone)]
#[func(pub const fn cost_cookies(self) -> f64)]
pub enum Research {
    #[assoc(cost_cookies = 1.0 * num::QUADRILLION)]
    BingoCenterResearchFacility,
    #[assoc(cost_cookies = 1.0 * num::QUADRILLION)]
    SpecializedChocolateChips,
    #[assoc(cost_cookies = 2.0 * num::QUADRILLION)]
    DesignerCocoaBeans,
    #[assoc(cost_cookies = 4.0 * num::QUADRILLION)]
    RitualRollingPins,
    #[assoc(cost_cookies = 8.0 * num::QUADRILLION)]
    UnderworldOvens,
    #[assoc(cost_cookies = 16.0 * num::QUADRILLION)]
    OneMind,
    #[assoc(cost_cookies = 32.0 * num::QUADRILLION)]
    ExoticNuts,
    #[assoc(cost_cookies = 64.0 * num::QUADRILLION)]
    CommunalBrainsweep,
    #[assoc(cost_cookies = 128.0 * num::QUADRILLION)]
    ArcaneSugar,
    #[assoc(cost_cookies = 256.0 * num::QUADRILLION)]
    ElderPact,
    #[assoc(cost_cookies = 2889.0 * num::TRILLION)]
    SacrificialRollingPins,
}

impl Research {
    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.cost_cookies())
    }

    pub fn req(&self) -> Req {
        match self {
            Self::BingoCenterResearchFacility => Req::All(&[
                Req::Achievement(Achievement::Elder),
                Req::BuildingCountMin(Building::Grandma, 6),
            ]),
            _ => Req::ResearchCompleted(Cmp::AboveOrEq(*self as u8)),
        }
    }

    pub fn buy(&self, state: &mut State) {
        match self {
            Self::BingoCenterResearchFacility => {
                state.buildings.set_grandma_has_bingo_center_4x(true);
            }
            Self::SpecializedChocolateChips => {
                state.grandmapocalypse.add_cps_mult(1.01);
            }
            Self::DesignerCocoaBeans => {
                state.grandmapocalypse.add_cps_mult(1.02);
            }
            Self::RitualRollingPins => {}
            Self::UnderworldOvens => {
                state.grandmapocalypse.add_cps_mult(1.03);
            }
            Self::OneMind => {
                state.buildings.modify(Building::Grandma, |b| {
                    b.addl_cps_per_owned_building
                        .push((Building::Grandma, 0.02));
                });

                state
                    .grandmapocalypse
                    .set_phase(GrandmapocalypsePhase::Awoken);
            }
            Self::ExoticNuts => {
                state.grandmapocalypse.add_cps_mult(1.04);
            }
            Self::CommunalBrainsweep => {
                state.buildings.modify(Building::Grandma, |b| {
                    b.addl_cps_per_owned_building
                        .push((Building::Grandma, 0.02));
                });

                state
                    .grandmapocalypse
                    .set_phase(GrandmapocalypsePhase::Displeased);
            }
            Self::ArcaneSugar => {
                state.grandmapocalypse.add_cps_mult(1.05);
            }
            Self::ElderPact => {
                state.buildings.modify(Building::Grandma, |b| {
                    b.addl_cps_per_owned_building.push((Building::Portal, 0.05));
                });

                state
                    .grandmapocalypse
                    .set_phase(GrandmapocalypsePhase::Angered);
            }
            Self::SacrificialRollingPins => {}
        }

        if !matches!(self, Self::SacrificialRollingPins) {
            state.research.start();
        }
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        use UpgradeEffectInfo::Research as R;
        use UpgradeInfoEffectResearch::*;
        use UpgradeInfoEffectResearchWarning::*;

        match self {
            Self::BingoCenterResearchFacility => R {
                effect: StartAndGrandmaCpsMult(4.0),
                warning: None,
            },
            Self::SpecializedChocolateChips => R {
                effect: CpsMultiplierPercent(0.01),
                warning: None,
            },
            Self::DesignerCocoaBeans => R {
                effect: CpsMultiplierPercent(0.02),
                warning: None,
            },
            Self::RitualRollingPins => R {
                effect: GrandmaCpsDouble,
                warning: None,
            },
            Self::UnderworldOvens => R {
                effect: CpsMultiplierPercent(0.03),
                warning: None,
            },
            Self::OneMind => R {
                effect: GrandmaGainsCpsPerBuilding(Building::Grandma, 0.02),
                warning: Some(One),
            },
            Self::ExoticNuts => R {
                effect: CpsMultiplierPercent(0.04),
                warning: None,
            },
            Self::CommunalBrainsweep => R {
                effect: GrandmaGainsCpsPerBuilding(Building::Grandma, 0.02),
                warning: Some(Two),
            },
            Self::ArcaneSugar => R {
                effect: CpsMultiplierPercent(0.05),
                warning: None,
            },
            Self::ElderPact => R {
                effect: GrandmaGainsCpsPerBuilding(Building::Portal, 0.05),
                warning: Some(Three),
            },
            Self::SacrificialRollingPins => R {
                effect: ElderPledgesLastTwiceAsLong,
                warning: None,
            },
        }
    }
}
