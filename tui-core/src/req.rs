use crate::{Building, State};

#[derive(Debug)]
pub enum Req {
    CookiesAbove(f64),
    CookiesAboveOrEq(f64),
    CookiesBelow(f64),
    CookiesBelowOrEq(f64),
    CookiesRange(f64, f64),
    BuildingCountMin(Building, u16),
    Custom(fn(&State) -> bool),
    Any(&'static [Req]),
    AnyBox(Box<[Req]>),
    All(&'static [Req]),
    AllBox(Box<[Req]>),
}

impl Req {
    pub fn check(&self, state: &State) -> bool {
        match self {
            Self::CookiesAbove(v) => state.cookies > *v,
            Self::CookiesAboveOrEq(v) => state.cookies >= *v,
            Self::CookiesBelow(v) => state.cookies < *v,
            Self::CookiesBelowOrEq(v) => state.cookies <= *v,
            Self::CookiesRange(a, b) => (*a..*b).contains(&state.cookies),
            Self::BuildingCountMin(b, c) => state.buildings.get(*b).count >= *c,
            Self::Custom(f) => f(state),
            Self::Any(reqs) => reqs.iter().any(|r| r.check(state)),
            Self::AnyBox(reqs) => reqs.iter().any(|r| r.check(state)),
            Self::All(reqs) => reqs.iter().all(|r| r.check(state)),
            Self::AllBox(reqs) => reqs.iter().all(|r| r.check(state)),
        }
    }
}
