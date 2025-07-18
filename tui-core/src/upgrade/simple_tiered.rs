use crate::Building;
use cookie_clicker_tui_utils::num;

#[derive(Debug)]
pub struct SimpleTieredUpgrade {
    building: Building,
    template: SimpleTieredUpgradeTemplate,
}

impl SimpleTieredUpgrade {
    pub fn new(building: Building, index: usize) -> Self {
        Self {
            building,
            template: SimpleTieredUpgradeTemplate::variants_array(building)[index],
        }
    }

    pub fn building(&self) -> Building {
        self.building
    }

    pub fn building_req(&self) -> u16 {
        self.template.building_req
    }

    pub fn cost(&self) -> f64 {
        self.building.base_cost() * self.template.cost_mult
    }
}

#[derive(Debug, Copy, Clone)]
struct SimpleTieredUpgradeTemplate {
    index: usize,
    building_req: u16,
    cost_mult: f64,
}

impl SimpleTieredUpgradeTemplate {
    const VARIANT_COUNT: usize = 15;

    const VARIANTS_CURSOR: [Self; Self::VARIANT_COUNT] = [
        Self::new(0, 1, num::SIX_REPEATING),
        Self::new(1, 1, num::THREE_THREE_REPEATING),
        Self::new(2, 10, num::SIX_SIX_SIX_REPEATING),
        Self::new(3, 25, num::SIX_REPEATING * num::THOUSAND),
        Self::new(4, 50, num::SIX_SIX_SIX_REPEATING * num::THOUSAND),
        Self::new(5, 100, num::SIX_REPEATING * num::MILLION),
        Self::new(6, 150, num::SIX_SIX_REPEATING * num::MILLION),
        Self::new(7, 200, num::SIX_SIX_SIX_REPEATING * num::MILLION),
        Self::new(8, 250, num::SIX_SIX_SIX_REPEATING * num::BILLION),
        Self::new(9, 300, num::SIX_SIX_SIX_REPEATING * num::TRILLION),
        Self::new(10, 350, num::SIX_SIX_SIX_REPEATING * num::QUADRILLION),
        Self::new(11, 400, num::SIX_SIX_SIX_REPEATING * num::QUINTILLION),
        Self::new(12, 450, num::SIX_SIX_SIX_REPEATING * num::SEXTILLION),
        Self::new(13, 500, num::SIX_SIX_SIX_REPEATING * num::SEPTILLION),
        Self::new(14, 550, num::SIX_SIX_SIX_REPEATING * num::OCTILLION),
    ];

    const VARIANTS_NON_CURSOR: [Self; Self::VARIANT_COUNT] = [
        Self::new(0, 1, 10.0),
        Self::new(1, 5, 50.0),
        Self::new(2, 25, 500.0),
        Self::new(3, 50, 50.0 * num::THOUSAND),
        Self::new(4, 100, 5.0 * num::MILLION),
        Self::new(5, 150, 500.0 * num::MILLION),
        Self::new(6, 200, 500.0 * num::BILLION),
        Self::new(7, 250, 500.0 * num::TRILLION),
        Self::new(8, 300, 500.0 * num::QUADRILLION),
        Self::new(9, 350, 500.0 * num::QUINTILLION),
        Self::new(10, 400, 5.0 * num::SEPTILLION),
        Self::new(11, 450, 50.0 * num::OCTILLION),
        Self::new(12, 500, 500.0 * num::NONILLION),
        Self::new(13, 550, 5.0 * num::UNDECILLION),
        Self::new(14, 600, 50.0 * num::DUODECILLION),
    ];

    const fn new(index: usize, building_req: u16, cost_mult: f64) -> Self {
        Self {
            index,
            building_req,
            cost_mult,
        }
    }

    const fn variants_array(building: Building) -> [Self; Self::VARIANT_COUNT] {
        if matches!(building, Building::Cursor) {
            Self::VARIANTS_CURSOR
        } else {
            Self::VARIANTS_NON_CURSOR
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Building;
    use approx_eq_trait::assert_approx_eq_slice;
    use cookie_clicker_tui_utils::num::*;

    fn prices_for(b: Building) -> [f64; 15] {
        std::array::from_fn(|i| SimpleTieredUpgrade::new(b, i).cost())
    }

    #[test]
    fn cursors() {
        assert_approx_eq_slice!(
            prices_for(Building::Cursor),
            [
                100.0,
                500.0,
                10.0 * THOUSAND,
                100.0 * THOUSAND,
                10.0 * MILLION,
                100.0 * MILLION,
                1.0 * BILLION,
                10.0 * BILLION,
                10.0 * TRILLION,
                10.0 * QUADRILLION,
                10.0 * QUINTILLION,
                10.0 * SEXTILLION,
                10.0 * SEPTILLION,
                10.0 * OCTILLION,
                10.0 * NONILLION,
            ]
        )
    }

    #[test]
    fn farms() {
        assert_approx_eq_slice!(
            prices_for(Building::Farm),
            [
                11.0 * THOUSAND,
                55.0 * THOUSAND,
                550.0 * THOUSAND,
                55.0 * MILLION,
                5.5 * BILLION,
                550.0 * BILLION,
                550.0 * TRILLION,
                550.0 * QUADRILLION,
                550.0 * QUINTILLION,
                550.0 * SEXTILLION,
                5.5 * OCTILLION,
                55.0 * NONILLION,
                550.0 * DECILLION,
                5.5 * DUODECILLION,
                55.0 * TREDECILLION
            ]
        );
    }

    #[test]
    fn factories() {
        assert_approx_eq_slice!(
            prices_for(Building::Factory),
            [
                1.3 * MILLION,
                6.5 * MILLION,
                65.0 * MILLION,
                6.5 * BILLION,
                650.0 * BILLION,
                65.0 * TRILLION,
                65.0 * QUADRILLION,
                65.0 * QUINTILLION,
                65.0 * SEXTILLION,
                65.0 * SEPTILLION,
                650.0 * OCTILLION,
                6.5 * DECILLION,
                65.0 * UNDECILLION,
                650.0 * DUODECILLION,
                6.5 * QUATTORDECILLION,
            ]
        );
    }
}
