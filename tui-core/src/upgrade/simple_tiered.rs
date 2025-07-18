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
        Self::new(1, 10.),
        Self::new(5, 50.),
        Self::new(25, 500.),
        Self::new(50, 50. * num::THOUSAND),
        Self::new(100, 5. * num::MILLION),
        Self::new(150, 500. * num::MILLION),
        Self::new(200, 500. * num::BILLION),
        Self::new(250, 500. * num::TRILLION),
        Self::new(300, 500. * num::QUADRILLION),
        Self::new(350, 500. * num::QUINTILLION),
        Self::new(400, 5. * num::SEPTILLION),
        Self::new(450, 50. * num::OCTILLION),
        Self::new(500, 500. * num::NONILLION),
        Self::new(550, 5. * num::UNDECILLION),
        Self::new(600, 50. * num::DUODECILLION),
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
                11. * THOUSAND,
                55. * THOUSAND,
                550. * THOUSAND,
                55. * MILLION,
                5.5 * BILLION,
                550. * BILLION,
                550. * TRILLION,
                550. * QUADRILLION,
                550. * QUINTILLION,
                550. * SEXTILLION,
                5.5 * OCTILLION,
                55. * NONILLION,
                550. * DECILLION,
                5.5 * DUODECILLION,
                55. * TREDECILLION
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
                65. * MILLION,
                6.5 * BILLION,
                650. * BILLION,
                65. * TRILLION,
                65. * QUADRILLION,
                65. * QUINTILLION,
                65. * SEXTILLION,
                65. * SEPTILLION,
                650. * OCTILLION,
                6.5 * DECILLION,
                65. * UNDECILLION,
                650. * DUODECILLION,
                6.5 * QUATTORDECILLION,
            ]
        );
    }
}
