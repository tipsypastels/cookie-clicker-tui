use crate::calc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Grandmapocalypse {
    phase: Option<GrandmapocalypsePhase>,
    cps_mults: Vec<f64>,
}

impl Grandmapocalypse {
    pub(crate) fn new() -> Self {
        Self {
            phase: None,
            cps_mults: Vec::new(),
        }
    }

    pub fn phase(&self) -> Option<GrandmapocalypsePhase> {
        self.phase
    }

    pub fn cps_mults(&self) -> &[f64] {
        &self.cps_mults
    }

    pub(crate) fn set_phase(&mut self, phase: GrandmapocalypsePhase) {
        self.phase = Some(phase)
    }

    pub(crate) fn add_cps_mult(&mut self, mult: f64) {
        self.cps_mults.push(mult);
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrandmapocalypsePhase {
    Awoken,
    Displeased,
    Angered,
}
