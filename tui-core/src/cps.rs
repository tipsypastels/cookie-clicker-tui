use crate::{Changeset, State, calc};
use cookie_clicker_tui_utils::refresh::Refresh;

#[derive(Debug)]
pub struct Cps {
    #[allow(unused)]
    pub base: f64,
    pub total: f64,
    #[allow(unused)]
    pub wrinkled: f64,
    pub debuff_ratio: f64,
    refresh: Refresh,
}

impl Cps {
    pub fn new(state: &State) -> Self {
        let calc = calc::cps(state);
        Self {
            base: calc.base,
            total: calc.total,
            wrinkled: calc.wrinkled,
            debuff_ratio: calc.debuff_ratio,
            refresh: Refresh::new(3.0),
        }
    }

    pub fn tick(&mut self, state: &State, changeset: &Changeset) {
        if self.refresh.finish() || changeset.cps {
            *self = Self::new(state);
        }
    }
}
