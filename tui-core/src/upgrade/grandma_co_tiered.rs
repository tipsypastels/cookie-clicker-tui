use crate::{State, building::Building, req::Req};

const SKIP: usize = 2; // cursors and grandmas
const COST_MULT: f64 = 50.0;

#[rustfmt::skip]
const LABELS: [&str; Building::VARIANT_COUNT - SKIP] = ["Farmer grandmas", "Miner grandmas", "Worker grandmas", "Banker grandmas", "Priestess grandmas", "Witch grandmas", "Cosmic grandmas", "Transmuted grandmas", "Altered grandmas", "Grandmas' grandmas", "Antigrandmas", "Rainbow grandmas", "Lucky grandmas", "Metagrandmas", "Binary grandmas", "Alternate grandmas", "Brainy grandmas", "Clone grandmas"];

#[derive(Debug)]
pub struct GrandmaCoTieredUpgrade {
    building: Building,
}

impl GrandmaCoTieredUpgrade {
    pub fn all() -> impl Iterator<Item = Self> {
        Building::variants()
            .skip(SKIP)
            .map(|building| Self { building })
    }

    pub fn without_taken(state: &State) -> impl Iterator<Item = Self> {
        Self::all().filter(|u| {
            !state
                .buildings
                .get(u.building)
                .has_grandma_co_tiered_upgrade
        })
    }

    pub fn req(&self) -> Req {
        Req::AllBox(Box::new([
            Req::BuildingCountMin(self.building, 15),
            Req::BuildingCountMin(Building::Grandma, 1),
        ]))
    }

    pub fn cost(&self) -> f64 {
        self.building.base_cost() * COST_MULT
    }

    pub fn label(&self) -> &'static str {
        LABELS[self.building as usize - SKIP]
    }

    pub fn description(&self) -> String {
        let num_req = crate::calc::grandma_co_tiered_upgrade_num_req_for_1p(self.building);
        format!(
            "2x {grandma} cps, +1% cps / {num_req} {grandmas}",
            grandma = Building::Grandma.name_lower(),
            grandmas = Building::Grandma.name_lower_pluralized(num_req as _)
        )
    }

    pub fn buy(&self, state: &mut State) {
        state
            .buildings
            .modify(self.building, |b| b.has_grandma_co_tiered_upgrade = true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq_trait::assert_approx_eq_slice;
    use cookie_clicker_tui_utils::num::*;

    #[test]
    fn building_prices() {
        assert_approx_eq_slice!(
            GrandmaCoTieredUpgrade::all()
                .map(|u| u.cost())
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
            GrandmaCoTieredUpgrade::all()
                .map(|u| u.label())
                .collect::<Vec<_>>(),
            LABELS,
        )
    }
}
