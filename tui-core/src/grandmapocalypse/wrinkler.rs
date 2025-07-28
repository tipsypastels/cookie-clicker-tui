use super::GrandmapocalypsePhase;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

const DEFAULT_MAX_SIZE: usize = 10;
#[cfg(debug_assertions)]
const DEFAULT_ODDS_PER_SPOT_PER_PHASE: f64 = 0.001;
#[cfg(not(debug_assertions))]
const DEFAULT_ODDS_PER_SPOT_PER_PHASE: f64 = 0.00001;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wrinklers {
    list: Vec<Wrinkler>,
    max_size: usize,
    odds_per_spot_per_phase: f64,
}

impl Wrinklers {
    pub(crate) fn new() -> Self {
        Self {
            list: Vec::new(),
            max_size: DEFAULT_MAX_SIZE,
            odds_per_spot_per_phase: DEFAULT_ODDS_PER_SPOT_PER_PHASE,
        }
    }

    pub(crate) fn tick(&mut self, phase: GrandmapocalypsePhase) {
        let available = self.available_size();
        if available > 0
            && rand::random::<f64>()
                <= self.odds_per_spot_per_phase * available as f64 * phase as u8 as f64
        {
            self.list.push(Wrinkler {});
        }
    }

    pub(crate) fn pop_all(&mut self) {
        self.list.clear();
    }

    fn available_size(&self) -> usize {
        self.max_size - self.list.len()
    }
}

impl Deref for Wrinklers {
    type Target = [Wrinkler];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wrinkler {}
