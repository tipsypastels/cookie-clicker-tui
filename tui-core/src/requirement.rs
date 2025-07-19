use crate::{Building, State};
use std::ops::Deref;

#[derive(Debug)]
pub struct Requirement(Inner);

impl Requirement {
    pub const fn cookies_above(v: f64) -> Self {
        Self(Inner::CookiesBound(v, true, false))
    }

    pub const fn cookies_above_or_eq(v: f64) -> Self {
        Self(Inner::CookiesBound(v, true, true))
    }

    pub const fn cookies_below(v: f64) -> Self {
        Self(Inner::CookiesBound(v, false, false))
    }

    pub const fn cookies_below_or_eq(v: f64) -> Self {
        Self(Inner::CookiesBound(v, false, true))
    }

    pub const fn cookies_range(a: f64, b: f64) -> Self {
        Self(Inner::CookiesRange(a, b))
    }

    pub const fn building_count_min(building: Building, count: u16) -> Self {
        Self(Inner::BuildingCountMin(building, count))
    }

    pub const fn custom(f: fn(&State) -> bool) -> Self {
        Self(Inner::Custom(f))
    }

    pub const fn any(slice: &'static [Requirement]) -> Self {
        Self(Inner::Any(List::Slice(slice)))
    }

    pub fn any_rc(rc: impl Into<Box<[Requirement]>>) -> Self {
        Self(Inner::Any(List::Box(rc.into())))
    }

    pub const fn all(slice: &'static [Requirement]) -> Self {
        Self(Inner::All(List::Slice(slice)))
    }

    pub fn all_rc(rc: impl Into<Box<[Requirement]>>) -> Self {
        Self(Inner::All(List::Box(rc.into())))
    }

    pub fn check(&self, state: &State) -> bool {
        self.0.check(state)
    }
}

#[derive(Debug)]
enum Inner {
    CookiesBound(f64, bool, bool),
    CookiesRange(f64, f64),
    BuildingCountMin(Building, u16),
    Custom(fn(&State) -> bool),
    Any(List<Requirement>),
    All(List<Requirement>),
}

impl Inner {
    fn check(&self, state: &State) -> bool {
        match self {
            Self::CookiesBound(v, true, false) => state.cookies > *v,
            Self::CookiesBound(v, true, true) => state.cookies >= *v,
            Self::CookiesBound(v, false, false) => state.cookies < *v,
            Self::CookiesBound(v, false, true) => state.cookies <= *v,
            Self::CookiesRange(a, b) => (*a..*b).contains(&state.cookies),
            Self::BuildingCountMin(b, c) => state.buildings.get(*b).count >= *c,
            Self::Custom(f) => f(state),
            Self::Any(list) => list.iter().any(|r| r.0.check(state)),
            Self::All(list) => list.iter().all(|r| r.0.check(state)),
        }
    }
}

#[derive(Debug)] // cow<[T]> but without the clone bound
enum List<T: 'static> {
    Slice(&'static [T]),
    Box(Box<[T]>),
}

impl<T: 'static> Deref for List<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Slice(s) => s,
            Self::Box(b) => b,
        }
    }
}
