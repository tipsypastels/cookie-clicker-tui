use super::effect_info::UpgradeEffectInfo;
use crate::{
    Achievement, Building, Cost, State,
    req::{Comparator, Req},
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
            other => Req::ResearchCompleted(Comparator::AboveOrEq(*other as u8)),
        }
    }

    pub fn buy(&self, state: &mut State) {
        if !matches!(self, Self::SacrificialRollingPins) {
            state.research.start();
        }
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        todo!()
    }
}
