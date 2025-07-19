use crate::{Building, Computed, State};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Requirement {
    CookiesBelow(f64),
    CookiesAbove(f64),
    CookiesRange(f64, f64),
    BuildingCountMin(Building, u16),
    Fn(fn(&State, &Computed) -> bool),
    Any(Cow<'static, [Self]>),
    All(Cow<'static, [Self]>),
}

impl Requirement {
    pub fn check(&self, state: &State, computed: &Computed) -> bool {
        match self {
            Self::CookiesBelow(max) => state.cookies < *max,
            Self::CookiesAbove(min) => state.cookies > *min,
            Self::CookiesRange(min, max) => (*min..*max).contains(&state.cookies),
            Self::BuildingCountMin(building, count) => {
                state.buildings.state(*building).count > *count
            }
            Self::Fn(f) => f(state, computed),
            Self::Any(slice) => slice.iter().any(|r| r.check(state, computed)),
            Self::All(slice) => slice.iter().all(|r| r.check(state, computed)),
        }
    }
}
