use crate::cps::Cps;
use cookie_clicker_tui_utils::{frames::FPS, refresh::Refresh};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookies {
    current: f64,
    all_time: f64,
    all_time_from_clicking: f64,
    #[serde(skip, default = "GainBulk::new")]
    gain_bulk: GainBulk,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            all_time: 0.0,
            all_time_from_clicking: 0.0,
            gain_bulk: GainBulk::new(),
        }
    }

    pub fn tick(&mut self, cps: &Cps) {
        self.gain(cps.total / FPS);
        self.gain_bulk.tick();
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

    pub fn enqueued_gain_bulk(&self) -> Option<f64> {
        self.gain_bulk.enqueued()
    }

    pub fn gain(&mut self, amount: f64) {
        self.current += amount;
        self.all_time += amount;
    }

    pub fn gain_from_clicking(&mut self, amount: f64) {
        self.gain(amount);
        self.all_time_from_clicking += amount;
    }

    pub fn gain_bulk(&mut self, amount: f64) {
        self.gain(amount);
        self.gain_bulk.enqueue(amount);
    }

    pub fn lose(&mut self, amount: f64) {
        self.current -= amount;
    }
}

#[derive(Debug)]
struct GainBulk {
    display_queue: VecDeque<f64>,
    refresh: Refresh,
}

impl GainBulk {
    fn new() -> Self {
        Self {
            display_queue: VecDeque::new(),
            refresh: Refresh::new(3.0),
        }
    }

    fn tick(&mut self) {
        if !self.display_queue.is_empty() && self.refresh.finish() {
            self.display_queue.pop_front();
        }
    }

    fn enqueued(&self) -> Option<f64> {
        self.display_queue.front().copied()
    }

    fn enqueue(&mut self, amount: f64) {
        self.display_queue.push_back(amount);
    }
}
