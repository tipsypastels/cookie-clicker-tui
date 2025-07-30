mod wrinkler;

pub use self::wrinkler::{Wrinkler, Wrinklers};

use self::Mode::*;
use crate::{Changeset, cookies::Cookies, cps::Cps};
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
    appeased_permanently_ever: bool,
    appeased_duration: f64,
    cps_mults: Vec<f64>,
    wrinklers: Wrinklers,
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
            appeased_permanently_ever: false,
            appeased_duration: DEFAULT_APPEASED_DURATION_SECS,
            cps_mults: Vec::new(),
            wrinklers: Wrinklers::new(),
        }
    }

    pub(crate) fn tick(
        &mut self,
        grandmas_count: u16,
        cps: &Cps,
        cookies: &mut Cookies,
        changeset: &mut Changeset,
    ) {
        match (grandmas_count, &mut self.mode) {
            (
                0,
                Phase(phase)
                | Appeased {
                    return_to: phase, ..
                },
            ) if changeset.grandmas_count => {
                self.wrinklers.pop_all(cookies, changeset);
                self.mode = NoGrandmas { return_to: *phase };
            }
            (n, NoGrandmas { return_to: phase }) if n > 0 && changeset.grandmas_count => {
                self.mode = Phase(*phase);
            }
            (_, Phase(phase)) => {
                self.wrinklers.tick(*phase, cps, changeset);
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

    pub fn wrinklers(&self) -> &Wrinklers {
        &self.wrinklers
    }

    pub(crate) fn wrinklers_mut(&mut self) -> &mut Wrinklers {
        &mut self.wrinklers
    }

    pub(crate) fn appeased_temporarily_times(&self) -> usize {
        self.appeased_temporarily_times
    }

    pub(crate) fn appeased_permanently_ever(&self) -> bool {
        self.appeased_permanently_ever
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

    pub(crate) fn appease_temporarily(&mut self, cookies: &mut Cookies, changeset: &mut Changeset) {
        match &mut self.mode {
            Phase(phase) => {
                self.wrinklers.pop_all(cookies, changeset);
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

    pub(crate) fn appease_permanently(&mut self, cookies: &mut Cookies, changeset: &mut Changeset) {
        match &mut self.mode {
            Phase(phase) => {
                self.wrinklers.pop_all(cookies, changeset);
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
            _ => return,
        }
        self.appeased_permanently_ever = true;
    }

    pub(crate) fn unappease(&mut self, changeset: &mut Changeset) {
        if let Mode::Appeased { return_to, .. } = self.mode {
            self.mode = Phase(return_to);
            changeset.cps = true;
        }
    }

    pub(crate) fn modify_appeased_duration(&mut self, mut f: impl FnMut(&mut f64)) {
        f(&mut self.appeased_duration);

        if let Mode::Appeased { refresh, .. } = &mut self.mode {
            refresh.modify(f);
        }
    }

    pub(crate) fn add_cps_mult(&mut self, mult: f64, changeset: &mut Changeset) {
        self.cps_mults.push(mult);
        changeset.cps = true;
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrandmapocalypsePhase {
    Awoken,
    Displeased,
    Angered,
}

impl GrandmapocalypsePhase {
    fn wrinkler_spawn_mult(self) -> f64 {
        (self as u8 + 1) as f64
    }
}
