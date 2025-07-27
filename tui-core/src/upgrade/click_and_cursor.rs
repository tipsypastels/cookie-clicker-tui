use super::effect_info::UpgradeEffectInfo;
use crate::{
    Building, Cost, State,
    req::{Cmp, Req},
};
use cookie_clicker_tui_utils::num;

pub struct ClickAndCursor {
    mode: ClickAndCursorMode,
    building_req: u16,
    cost_mult: f64,
}

pub enum ClickAndCursorMode {
    Double,
    ThousandFingers,
    ThousandFingersMult(f64),
}

impl ClickAndCursor {
    pub const fn new(index: usize, mode: ClickAndCursorMode) -> Self {
        let (building_req, cost_mult) = TEMPLATES[index];
        Self {
            mode,
            building_req,
            cost_mult,
        }
    }
    pub fn cost(&self) -> Cost {
        Cost::Cookies(Building::Cursor.base_cost() * self.cost_mult)
    }

    pub fn req(&self) -> Req {
        Req::BuildingCount(Building::Cursor, Cmp::AboveOrEq(self.building_req))
    }

    pub fn buy(&self, state: &mut State) {
        match self.mode {
            ClickAndCursorMode::Double => {
                state
                    .buildings
                    .modify_tiered_upgrade_count(Building::Cursor, |c| *c += 1);
            }
            ClickAndCursorMode::ThousandFingers => {
                state
                    .thousand_fingers
                    .enable(&mut state.buildings, &mut state.click);
            }
            ClickAndCursorMode::ThousandFingersMult(mult) => {
                state
                    .thousand_fingers
                    .multiply(mult, &mut state.buildings, &mut state.click);
            }
        }
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        match self.mode {
            ClickAndCursorMode::Double => UpgradeEffectInfo::Tiered(Building::Cursor),
            ClickAndCursorMode::ThousandFingers => UpgradeEffectInfo::ThousandFingers,
            ClickAndCursorMode::ThousandFingersMult(mult) => {
                UpgradeEffectInfo::ThousandFingersMult(mult)
            }
        }
    }
}

const TEMPLATES: [(u16, f64); 15] = [
    (1, num::SIX_REPEATING),
    (1, num::THREE_THREE_REPEATING),
    (10, num::SIX_SIX_SIX_REPEATING),
    (25, num::SIX_REPEATING * num::THOUSAND),
    (50, num::SIX_SIX_SIX_REPEATING * num::THOUSAND),
    (100, num::SIX_REPEATING * num::MILLION),
    (150, num::SIX_SIX_REPEATING * num::MILLION),
    (200, num::SIX_SIX_SIX_REPEATING * num::MILLION),
    (250, num::SIX_SIX_SIX_REPEATING * num::BILLION),
    (300, num::SIX_SIX_SIX_REPEATING * num::TRILLION),
    (350, num::SIX_SIX_SIX_REPEATING * num::QUADRILLION),
    (400, num::SIX_SIX_SIX_REPEATING * num::QUINTILLION),
    (450, num::SIX_SIX_SIX_REPEATING * num::SEXTILLION),
    (500, num::SIX_SIX_SIX_REPEATING * num::SEPTILLION),
    (550, num::SIX_SIX_SIX_REPEATING * num::OCTILLION),
];
