use crate::{Building, Computed, State};

#[allow(unused)]
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

#[allow(unused)]
#[derive(Debug)]
pub enum LateReq {
    Req(Req),
    CpsAbove(f64),
    Custom(fn(&State, &Computed) -> bool),
    Any(&'static [LateReq]),
    AnyBox(Box<[LateReq]>),
    All(&'static [LateReq]),
    AllBox(Box<[LateReq]>),
}

macro_rules! delegated_late_variants {
    ($($fn:ident($($arg:ident: $ty:ty),*);)*) => {
        $(#[allow(unused, non_snake_case)] pub const fn $fn($($arg: $ty),*) -> Self {
            Self::Req(Req::$fn($($arg),*))
        })*
    };
}

impl LateReq {
    delegated_late_variants! {
        CookiesAbove(v: f64);
        CookiesAboveOrEq(v: f64);
        CookiesBelow(v: f64);
        CookiesBelowOrEq(v: f64);
        CookiesRange(a: f64, b: f64);
        BuildingCountMin(b: Building, c: u16);
    }

    pub fn check(&self, state: &State, computed: &Computed) -> bool {
        match self {
            Self::Req(req) => req.check(state),
            Self::CpsAbove(v) => computed.cps > *v,
            Self::Custom(f) => f(state, computed),
            Self::Any(reqs) => reqs.iter().any(|r| r.check(state, computed)),
            Self::AnyBox(reqs) => reqs.iter().any(|r| r.check(state, computed)),
            Self::All(reqs) => reqs.iter().all(|r| r.check(state, computed)),
            Self::AllBox(reqs) => reqs.iter().all(|r| r.check(state, computed)),
        }
    }
}
