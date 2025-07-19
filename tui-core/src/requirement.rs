use crate::{Computed, State};

#[derive(Debug, Copy, Clone)]
pub enum Requirement {
    CookieUnder(f64),
    CookieAbove(f64),
    CookieRange(f64, f64),
    Fn(fn(&State, &Computed) -> bool),
    Any(&'static [Self]),
    All(&'static [Self]),
}

impl Requirement {
    pub(crate) fn check(&self, state: &State, computed: &Computed) -> bool {
        match self {
            Self::CookieUnder(max) => state.cookies < *max,
            Self::CookieAbove(min) => state.cookies > *min,
            Self::CookieRange(min, max) => (*min..*max).contains(&state.cookies),
            Self::Fn(f) => f(state, computed),
            Self::Any(slice) => slice.iter().any(|r| r.check(state, computed)),
            Self::All(slice) => slice.iter().all(|r| r.check(state, computed)),
        }
    }
}
