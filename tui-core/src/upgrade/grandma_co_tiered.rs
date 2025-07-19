use crate::building::Building;

const COST_MULT: f64 = 50.0;
const SKIP: usize = 2; // cursors and grandmas

#[rustfmt::skip]
const LABELS: [&str; Building::VARIANT_COUNT - SKIP] = ["Farmer grandmas", "Miner grandmas", "Worker grandmas", "Banker grandmas", "Priestess grandmas", "Witch grandmas", "Cosmic grandmas", "Transmuted grandmas", "Altered grandmas", "Grandmas' grandmas", "Antigrandmas", "Rainbow grandmas", "Lucky grandmas", "Metagrandmas", "Binary grandmas", "Alternate grandmas", "Brainy grandmas", "Clone grandmas"];

#[derive(Debug)]
pub struct GrandmaCoTieredUpgrade {
    building: Building,
}

impl GrandmaCoTieredUpgrade {
    pub const fn new(building: Building) -> Self {
        debug_assert!(!matches!(building, Building::Cursor | Building::Grandma));

        Self { building }
    }

    pub const fn building(&self) -> Building {
        self.building
    }

    pub const fn building_req(&self) -> (u16, u16) {
        (15, 1) // TODO: Make a consistent builder API for this.
    }

    pub const fn cost(&self) -> f64 {
        self.building.base_cost() * COST_MULT
    }

    pub const fn label(&self) -> &'static str {
        LABELS[self.building as usize - SKIP]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq_trait::assert_approx_eq_slice;
    use cookie_clicker_tui_utils::num::*;

    #[test]
    #[should_panic]
    fn panic_cursor() {
        GrandmaCoTieredUpgrade::new(Building::Cursor);
    }

    #[test]
    #[should_panic]
    fn panic_grandma() {
        GrandmaCoTieredUpgrade::new(Building::Grandma);
    }

    #[test]
    fn building_prices() {
        assert_approx_eq_slice!(
            Building::variants()
                .skip(SKIP)
                .map(|b| GrandmaCoTieredUpgrade::new(b).cost())
                .collect::<Vec<_>>(),
            [
                55.0 * THOUSAND,
                600.0 * THOUSAND,
                6.5 * MILLION,
                70.0 * MILLION,
                1.0 * BILLION,
                16.5 * BILLION,
                255.0 * BILLION,
                3.75 * TRILLION,
                50.0 * TRILLION,
                700.0 * TRILLION,
                8.5 * QUADRILLION,
                105.0 * QUADRILLION,
                1.3 * QUINTILLION,
                15.5 * QUINTILLION,
                3.55 * SEXTILLION,
                600.0 * SEXTILLION,
                95.0 * SEPTILLION,
                27.0 * OCTILLION,
            ],
        );
    }

    #[test]
    fn building_labels() {
        assert_eq!(
            Building::variants()
                .skip(SKIP)
                .map(|b| GrandmaCoTieredUpgrade::new(b).label())
                .collect::<Vec<_>>(),
            LABELS,
        )
    }
}
