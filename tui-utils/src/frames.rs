#![allow(deprecated)]

use std::fmt;

pub const FPS: f64 = 30.0;

#[deprecated]
pub struct RefreshClock<const SECONDS: u32> {
    ticks_until_refresh: u32,
}

impl<const SECONDS: u32> RefreshClock<SECONDS> {
    pub const TICKS: u32 = SECONDS * FPS as u32;

    pub fn new() -> Self {
        Self {
            ticks_until_refresh: Self::TICKS,
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }

    pub fn finish(&mut self) -> bool {
        if let Some(ticks_until_refresh) = self.ticks_until_refresh.checked_sub(1) {
            self.ticks_until_refresh = ticks_until_refresh;
            false
        } else {
            true
        }
    }
}

impl<const SECONDS: u32> Default for RefreshClock<SECONDS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SECONDS: u32> fmt::Debug for RefreshClock<SECONDS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ticks_until_refresh.fmt(f)
    }
}
