use crate::{State, cps::Cps};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub enum Cost {
    Cookies(f64),
    Dyn(CostDyn),
}

impl Cost {
    pub(crate) fn resolve(self, state: &State, cps: &Cps) -> CostResolved {
        match self {
            Self::Cookies(c) => CostResolved::Cookies(c),
            Self::Dyn(c) => (c.f)(state, cps),
        }
    }

    pub(crate) fn affordable(self, state: &State, cps: &Cps) -> bool {
        self.resolve(state, cps).affordable(state)
    }

    pub(crate) fn total_cmp(a: Self, b: Self, state: &State, cps: &Cps) -> Ordering {
        CostResolved::total_cmp(a.resolve(state, cps), b.resolve(state, cps))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum CostResolved {
    Cookies(f64),
}

impl CostResolved {
    pub(crate) fn affordable(self, state: &State) -> bool {
        match self {
            Self::Cookies(c) => c <= state.cookies.current(),
        }
    }

    pub(crate) fn total_cmp(a: Self, b: Self) -> Ordering {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum CostKind {
            Cookies,
        }
        impl From<CostResolved> for CostKind {
            fn from(cost: CostResolved) -> Self {
                match cost {
                    CostResolved::Cookies(_) => Self::Cookies,
                }
            }
        }
        CostKind::from(a)
            .cmp(&CostKind::from(b))
            .then_with(|| match (a, b) {
                (CostResolved::Cookies(a), CostResolved::Cookies(b)) => f64::total_cmp(&a, &b),
                #[allow(unreachable_patterns)] // for future expansion
                _ => unreachable!(),
            })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CostDyn {
    f: fn(&State, &Cps) -> CostResolved,
}

impl CostDyn {
    pub(crate) fn new(f: fn(&State, &Cps) -> CostResolved) -> Self {
        Self { f }
    }
}
