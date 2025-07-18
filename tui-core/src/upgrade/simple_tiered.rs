use cookie_clicker_tui_utils::num;

#[derive(Debug)]
pub struct SimpleTieredUpgrade {
    pub building_req: u16,
    pub cost_mult: f64,
    _priv: (),
}

impl SimpleTieredUpgrade {
    const VARIANT_COUNT: usize = 15;
    const VARIANTS: [Self; Self::VARIANT_COUNT] = [
        Self::new(1, 10.0),
        Self::new(5, 50.0),
        Self::new(25, 500.0),
        Self::new(50, 50.0 * num::THOUSAND),
        Self::new(100, 5.0 * num::MILLION),
        Self::new(150, 500.0 * num::MILLION),
        Self::new(200, 500.0 * num::BILLION),
        Self::new(250, 500.0 * num::TRILLION),
        Self::new(300, 500.0 * num::QUADRILLION),
        Self::new(350, 500.0 * num::QUINTILLION),
        Self::new(400, 5.0 * num::SEPTILLION),
        Self::new(450, 50.0 * num::OCTILLION),
        Self::new(500, 500.0 * num::NONILLION),
        Self::new(550, 5.0 * num::UNDECILLION),
        Self::new(600, 50.0 * num::DUODECILLION),
    ];

    const fn new(building_req: u16, cost_mult: f64) -> Self {
        Self {
            building_req,
            cost_mult,
            _priv: (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BuildingKind;
    use approx_eq_trait::assert_approx_eq_slice;
    use cookie_clicker_tui_utils::num::*;

    fn prices_for(b: BuildingKind) -> [f64; 15] {
        SimpleTieredUpgrade::VARIANTS.map(|v| b.base_cost() * v.cost_mult)
    }

    #[test]
    fn farms() {
        assert_approx_eq_slice!(
            prices_for(BuildingKind::Farm),
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
            prices_for(BuildingKind::Factory),
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
