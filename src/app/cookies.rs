use crate::{event::FPS, num::AsBigCountFmt};
use std::{cmp::Ordering, ops::RangeBounds};

#[derive(Debug, Copy, Clone)]
pub struct Cookies(f64);

impl Cookies {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn tick(&mut self, cps: f64) {
        self.0 += cps / FPS;
    }

    pub fn add(&mut self, cookies: f64) {
        self.0 += cookies;
    }

    pub fn sub(&mut self, cookies: f64) {
        self.0 -= cookies;
    }

    pub fn value(self) -> u128 {
        self.0.floor() as _
    }

    pub fn value_f64(self) -> f64 {
        self.0
    }

    pub fn in_range(self, range: &impl RangeBounds<u128>) -> bool {
        range.contains(&self.value())
    }
}

impl PartialEq<u128> for Cookies {
    fn eq(&self, other: &u128) -> bool {
        self.value().eq(other)
    }
}

impl PartialOrd<u128> for Cookies {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        self.value().partial_cmp(other)
    }
}

impl PartialEq<f64> for Cookies {
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f64> for Cookies {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl AsBigCountFmt for Cookies {
    fn as_big_count_fmt(&self) -> crate::num::BigCountFmt {
        self.value_f64().as_big_count_fmt()
    }
}
