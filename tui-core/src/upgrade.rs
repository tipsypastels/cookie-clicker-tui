mod effect_info;
mod grandma_co_tiered;
mod simple_tiered;

pub use self::effect_info::UpgradeEffectInfo;

use self::{grandma_co_tiered::GrandmaCoTieredUpgrade, simple_tiered::SimpleTieredUpgrade};
use crate::State;
use cookie_clicker_tui_utils::frames::RefreshClock;
use std::ops::Deref;

#[derive(Debug)]
pub struct Upgrades {
    list: Box<[Upgrade]>,
    refresh: RefreshClock<5>,
}

impl Upgrades {
    pub fn new(state: &State) -> Self {
        let list = Upgrade::unlocked(state);
        let refresh = RefreshClock::new();

        Self { list, refresh }
    }

    pub fn tick(&mut self, state: &State) {
        if self.refresh.finish() {
            *self = Self::new(state);
        }
    }
}

impl Deref for Upgrades {
    type Target = [Upgrade];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

#[derive(Debug)]
pub struct Upgrade(Inner);

#[derive(Debug)]
enum Inner {
    SimpleTiered(SimpleTieredUpgrade),
    GrandmaCoTiered(GrandmaCoTieredUpgrade),
}

impl Upgrade {
    fn unlocked(state: &State) -> Box<[Self]> {
        let mut out = Vec::new();

        macro_rules! extend {
            ($($type:ident),*$(,)?) => {
                $(out.extend(
                    $type::without_taken(state)
                        .filter(|u| u.req().check(state))
                        .map(|u| Self(Inner::from(u))),
                ));*
            };
        }

        extend! {
            SimpleTieredUpgrade,
            GrandmaCoTieredUpgrade,
        }

        out.sort_by(|a, b| f64::total_cmp(&a.cost(), &b.cost()));
        out.into()
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

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        match &self.0 {
            Inner::SimpleTiered(u) => u.effect_info(),
            Inner::GrandmaCoTiered(u) => u.effect_info(),
        }
    }

    pub(crate) fn buy(&self, state: &mut State) {
        match &self.0 {
            Inner::SimpleTiered(u) => u.buy(state),
            Inner::GrandmaCoTiered(u) => u.buy(state),
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
