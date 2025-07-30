use crate::frames::FPS;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Refresh {
    cur: f64,
    max: f64,
}

impl Refresh {
    pub fn new(max_secs: f64) -> Self {
        Self {
            cur: 0.0,
            max: max_secs * FPS,
        }
    }

    pub fn new_frames(max: f64) -> Self {
        Self { cur: 0.0, max }
    }

    pub fn cur(&self) -> f64 {
        self.cur
    }

    pub fn cur_secs(&self) -> f64 {
        self.cur / FPS
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn max_secs(&self) -> f64 {
        self.max / FPS
    }

    pub fn until_finish(&self) -> f64 {
        self.max - self.cur
    }

    pub fn until_finish_secs(&self) -> f64 {
        self.until_finish() / FPS
    }

    pub fn reset(&mut self) {
        self.cur = 0.0;
    }

    pub fn finish(&mut self) -> bool {
        let next = self.cur + 1.0;
        if next >= self.max {
            self.cur = 0.0;
            true
        } else {
            self.cur = next;
            false
        }
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut f64)) {
        f(&mut self.max);
    }
}

impl fmt::Debug for Refresh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..={}", self.cur, self.max)
    }
}

pub trait RefreshOptionExt {
    fn finish_and_set_none(&mut self) -> bool;
}

impl RefreshOptionExt for Option<Refresh> {
    fn finish_and_set_none(&mut self) -> bool {
        if let Some(refresh) = self.as_mut()
            && refresh.finish()
        {
            *self = None;
            true
        } else {
            false
        }
    }
}
