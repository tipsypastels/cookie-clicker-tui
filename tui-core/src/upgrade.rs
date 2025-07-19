mod grandma_co_tiered;
mod simple_tiered;

use self::{grandma_co_tiered::GrandmaCoTieredUpgrade, simple_tiered::SimpleTieredUpgrade};
use crate::State;

#[derive(Debug)]
pub struct Upgrade(Inner);

#[derive(Debug)]
enum Inner {
    SimpleTiered(SimpleTieredUpgrade),
    GrandmaCoTiered(GrandmaCoTieredUpgrade),
}

impl Upgrade {
    pub(crate) fn unlocked(state: &State) -> Vec<Self> {
        let mut out = Vec::new();

        macro_rules! extend {
            ($($type:ident),*$(,)?) => {
                $(out.extend(
                    $type::without_taken(state)
                        .filter(|u| u.requirement().check(state))
                        .map(|u| Self(Inner::from(u))),
                ));*
            };
        }

        extend! {
            SimpleTieredUpgrade,
            GrandmaCoTieredUpgrade,
        }

        out.sort_by(|a, b| f64::total_cmp(&a.cost(), &b.cost()));
        out
    }

    pub fn cost(&self) -> f64 {
        match &self.0 {
            Inner::SimpleTiered(u) => u.cost(),
            Inner::GrandmaCoTiered(u) => u.cost(),
        }
    }

    pub fn label(&self) -> &'static str {
        match &self.0 {
            Inner::SimpleTiered(u) => u.label(),
            Inner::GrandmaCoTiered(u) => u.label(),
        }
    }
}

impl From<SimpleTieredUpgrade> for Inner {
    fn from(upgrade: SimpleTieredUpgrade) -> Self {
        Self::SimpleTiered(upgrade)
    }
}

impl From<GrandmaCoTieredUpgrade> for Inner {
    fn from(upgrade: GrandmaCoTieredUpgrade) -> Self {
        Self::GrandmaCoTiered(upgrade)
    }
}
