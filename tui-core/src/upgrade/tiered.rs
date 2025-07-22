use super::effect_info::UpgradeEffectInfo;
use crate::{Building, Cost, State, req::Req};
use cookie_clicker_tui_utils::num;

pub struct Tiered {
    building: Building,
    building_req: u16,
    cost_mult: f64,
}

impl Tiered {
    pub const fn new(index: usize, building: Building) -> Self {
        let templates = if building.is_cursor() {
            TEMPLATES_CURSOR
        } else {
            TEMPLATES_NON_CURSOR
        };

        let (building_req, cost_mult) = templates[index];

        Self {
            building,
            building_req,
            cost_mult,
        }
    }

    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.building.base_cost() * self.cost_mult)
    }

    pub fn req(&self) -> Req {
        Req::BuildingCountMin(self.building, self.building_req)
    }

    pub fn buy(&self, state: &mut State) {
        state
            .buildings
            .modify(self.building, |b| b.tiered_upgrade_count += 1);
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        UpgradeEffectInfo::Tiered(self.building)
    }
}

const TEMPLATES_COUNT: usize = 15;

const TEMPLATES_CURSOR: [(u16, f64); TEMPLATES_COUNT] = [
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

const TEMPLATES_NON_CURSOR: [(u16, f64); TEMPLATES_COUNT] = [
    (1, 10.0),
    (5, 50.0),
    (25, 500.0),
    (50, 50.0 * num::THOUSAND),
    (100, 5.0 * num::MILLION),
    (150, 500.0 * num::MILLION),
    (200, 500.0 * num::BILLION),
    (250, 500.0 * num::TRILLION),
    (300, 500.0 * num::QUADRILLION),
    (350, 500.0 * num::QUINTILLION),
    (400, 5.0 * num::SEPTILLION),
    (450, 50.0 * num::OCTILLION),
    (500, 500.0 * num::NONILLION),
    (550, 5.0 * num::UNDECILLION),
    (600, 50.0 * num::DUODECILLION),
];
