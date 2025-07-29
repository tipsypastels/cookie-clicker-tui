use crate::{State, calc};

#[derive(Debug)]
pub struct Cps {
    #[allow(unused)]
    pub base: f64,
    pub total: f64,
    #[allow(unused)]
    pub wrinkled: f64,
}

impl Cps {
    pub fn new(state: &State) -> Self {
        let calc = calc::cps(state);
        Self {
            base: calc.base,
            total: calc.total,
            wrinkled: calc.wrinkled,
        }
    }

    pub fn recalc(&mut self, state: &State) {
        *self = Self::new(state);
    }
}
