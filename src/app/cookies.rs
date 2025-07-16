use crate::event::FPS;
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

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn in_range(self, range: &impl RangeBounds<f64>) -> bool {
        range.contains(&self.0)
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
