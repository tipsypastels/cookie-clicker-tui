use cookie_clicker_tui_utils::frames::FPS;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookies {
    current: f64,
    all_time: f64,
    all_time_from_clicking: f64,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            all_time: 0.0,
            all_time_from_clicking: 0.0,
        }
    }

    pub fn current(&self) -> f64 {
        self.current
    }

    pub fn all_time(&self) -> f64 {
        self.all_time
    }

    pub fn all_time_from_clicking(&self) -> f64 {
        self.all_time_from_clicking
    }

    pub fn gain(&mut self, amount: f64) {
        self.current += amount;
        self.all_time += amount;
    }

    pub fn gain_from_clicking(&mut self, amount: f64) {
        self.gain(amount);
        self.all_time_from_clicking += amount;
    }

    pub fn lose(&mut self, amount: f64) {
        self.current -= amount;
    }

    pub fn tick(&mut self, cps: f64) {
        self.gain(cps / FPS);
    }
}
