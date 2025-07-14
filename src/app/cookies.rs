use std::{cmp::Ordering, ops::RangeBounds};

use crate::event::FPS;

#[derive(Debug, Copy, Clone)]
pub struct Cookies(f64);

impl Cookies {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn tick(&mut self, cps: f64) {
        self.0 += cps / FPS;
    }

    pub fn add(&mut self, cookies: u32) {
        self.0 += cookies as f64;
    }

    pub fn sub(&mut self, cookies: u32) {
        self.0 -= cookies as f64;
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
