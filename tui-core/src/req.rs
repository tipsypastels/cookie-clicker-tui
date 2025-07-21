use crate::{Building, Computed, State};

#[allow(unused)]
pub enum Req {
    Cookies(Comparator<f64>),
    CookiesAllTime(Comparator<f64>),
    CookiesAllTimeFromClicking(Comparator<f64>),
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
            Self::Cookies(c) => c.check(state.cookies.current()),
            Self::CookiesAllTime(c) => c.check(state.cookies.all_time()),
            Self::CookiesAllTimeFromClicking(c) => c.check(state.cookies.all_time_from_clicking()),
            Self::BuildingCountMin(b, c) => state.buildings.count(*b) >= *c,
            Self::Custom(f) => f(state),
            Self::Any(reqs) => reqs.iter().any(|r| r.check(state)),
            Self::AnyBox(reqs) => reqs.iter().any(|r| r.check(state)),
            Self::All(reqs) => reqs.iter().all(|r| r.check(state)),
            Self::AllBox(reqs) => reqs.iter().all(|r| r.check(state)),
        }
    }
}

#[allow(unused)]
pub enum LateReq {
    Req(Req),
    Cps(Comparator<f64>),
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
        Cookies(c: Comparator<f64>);
        CookiesAllTime(c: Comparator<f64>);
        CookiesAllTimeFromClicking(c: Comparator<f64>);
        BuildingCountMin(b: Building, c: u16);
    }

    pub fn check(&self, state: &State, computed: &Computed) -> bool {
        match self {
            Self::Req(req) => req.check(state),
            Self::Cps(c) => c.check(computed.cps),
            Self::Custom(f) => f(state, computed),
            Self::Any(reqs) => reqs.iter().any(|r| r.check(state, computed)),
            Self::AnyBox(reqs) => reqs.iter().any(|r| r.check(state, computed)),
            Self::All(reqs) => reqs.iter().all(|r| r.check(state, computed)),
            Self::AllBox(reqs) => reqs.iter().all(|r| r.check(state, computed)),
        }
    }
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Comparator<T> {
    Above(T),
    AboveOrEq(T),
    Below(T),
    BelowOrEq(T),
    Range(T, T),
}

impl<T: PartialOrd> Comparator<T> {
    fn check(self, value: T) -> bool {
        match self {
            Self::Above(v) => value > v,
            Self::AboveOrEq(v) => value >= v,
            Self::Below(v) => value < v,
            Self::BelowOrEq(v) => value <= v,
            Self::Range(a, b) => (a..b).contains(&value),
        }
    }
}
