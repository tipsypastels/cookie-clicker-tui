use self::Mode::*;
use cookie_clicker_tui_utils::refresh::Refresh;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const DEFAULT_APPEASED_DURATION_SECS: f64 = 10.0;

#[cfg(not(debug_assertions))]
const DEFAULT_APPEASED_DURATION_SECS: f64 = 30.0 * 60.0;

#[derive(Serialize, Deserialize, Debug)]
pub struct Grandmapocalypse {
    #[serde(flatten)]
    mode: Mode,
    appeased_temporarily_times: usize,
    appeased_duration: f64,
    cps_mults: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Mode {
    Off,
    Phase(GrandmapocalypsePhase),
    NoGrandmas {
        return_to: GrandmapocalypsePhase,
    },
    Appeased {
        return_to: GrandmapocalypsePhase,
        temporary: bool,
        refresh: Refresh,
    },
}

impl Grandmapocalypse {
    pub(crate) fn new() -> Self {
        Self {
            mode: Off,
            appeased_temporarily_times: 0,
            appeased_duration: DEFAULT_APPEASED_DURATION_SECS,
            cps_mults: Vec::new(),
        }
    }

    pub(crate) fn tick(&mut self, grandma_count: u16) {
        match (grandma_count, &mut self.mode) {
            (
                0,
                Phase(phase)
                | Appeased {
                    return_to: phase, ..
                },
            ) => {
                self.mode = NoGrandmas { return_to: *phase };
            }
            (n, NoGrandmas { return_to: phase }) if n > 0 => {
                self.mode = Phase(*phase);
            }
            (
                _,
                Appeased {
                    temporary: true,
                    refresh,
                    return_to: phase,
                },
            ) => {
                if refresh.finish() {
                    self.mode = Phase(*phase);
                    self.appeased_temporarily_times =
                        self.appeased_temporarily_times.saturating_add(1);
                }
            }
            _ => {}
        }
    }

    pub fn phase(&self) -> Option<GrandmapocalypsePhase> {
        match self.mode {
            Mode::Phase(phase) => Some(phase),
            _ => None,
        }
    }

    pub fn is_phase(&self, phase: GrandmapocalypsePhase) -> bool {
        self.phase().is_some_and(|p| p == phase)
    }

    pub fn is_appeased(&self) -> bool {
        matches!(self.mode, Appeased { .. })
    }

    pub fn is_appeased_temporarily(&self) -> bool {
        matches!(
            self.mode,
            Appeased {
                temporary: true,
                ..
            }
        )
    }

    // TODO: This should affect CPS (0.95x).
    pub fn is_appeased_permanently(&self) -> bool {
        matches!(
            self.mode,
            Appeased {
                temporary: false,
                ..
            }
        )
    }

    pub fn is_no_grandmas(&self) -> bool {
        matches!(self.mode, NoGrandmas { .. })
    }

    pub(crate) fn appeased_temporarily_times(&self) -> usize {
        self.appeased_temporarily_times
    }

    pub(crate) fn cps_mults(&self) -> &[f64] {
        &self.cps_mults
    }

    pub(crate) fn set_phase(&mut self, phase: GrandmapocalypsePhase) {
        match &mut self.mode {
            Off => {
                self.mode = Phase(phase);
            }
            Phase(prev_phase) => {
                *prev_phase = phase;
            }
            NoGrandmas { return_to } | Appeased { return_to, .. } => {
                *return_to = phase;
            }
        }
    }

    pub(crate) fn appease_temporarily(&mut self) {
        match &mut self.mode {
            Phase(phase) => {
                self.mode = Appeased {
                    return_to: *phase,
                    temporary: true,
                    refresh: Refresh::new(self.appeased_duration),
                }
            }
            Appeased {
                temporary: temporary @ false,
                ..
            } => {
                *temporary = true;
            }
            _ => {}
        }
    }

    pub(crate) fn appease_permanently(&mut self) {
        match &mut self.mode {
            Phase(phase) => {
                self.mode = Appeased {
                    return_to: *phase,
                    temporary: false,
                    refresh: Refresh::new(self.appeased_duration),
                }
            }
            Appeased {
                temporary: temporary @ true,
                ..
            } => {
                *temporary = false;
            }
            _ => {}
        }
    }

    pub(crate) fn unappease(&mut self) {
        if let Mode::Appeased { return_to, .. } = self.mode {
            self.mode = Phase(return_to);
        }
    }

    pub(crate) fn modify_appeased_duration(&mut self, mut f: impl FnMut(&mut f64)) {
        f(&mut self.appeased_duration);

        if let Mode::Appeased { refresh, .. } = &mut self.mode {
            refresh.modify(f);
        }
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
