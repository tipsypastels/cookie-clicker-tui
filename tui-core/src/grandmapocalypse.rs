use cookie_clicker_tui_utils::refresh::Refresh;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const DEFAULT_APPEASED_DURATION_SECS: f64 = 10.0;

#[cfg(not(debug_assertions))]
const DEFAULT_APPEASED_DURATION_SECS: f64 = 30.0 * 60.0;

#[derive(Serialize, Deserialize, Debug)]
pub struct Grandmapocalypse {
    current: Current,
    appeased_temporarily_times: usize,
    appeased_duration: f64,
    cps_mults: Vec<f64>,
}

impl Grandmapocalypse {
    pub(crate) fn new() -> Self {
        Self {
            current: Current::None,
            appeased_temporarily_times: 0,
            appeased_duration: DEFAULT_APPEASED_DURATION_SECS,
            cps_mults: Vec::new(),
        }
    }

    pub(crate) fn tick(&mut self) {
        if let Current::AppeasedTemporarily { refresh } = &mut self.current
            && refresh.finish()
        {
            self.current = Current::Phase(GrandmapocalypsePhase::Angered);
            self.appeased_temporarily_times = self.appeased_temporarily_times.saturating_add(1);
        }
    }

    pub fn phase(&self) -> Option<GrandmapocalypsePhase> {
        match self.current {
            Current::Phase(phase) => Some(phase),
            _ => None,
        }
    }

    pub fn info(&self) -> Option<GrandmapocalypseInfo> {
        use self::{Current as C, GrandmapocalypseInfo as I, GrandmapocalypsePhase as P};
        match &self.current {
            C::None => None,
            C::Phase(P::Awoken) => Some(I::Awoken),
            C::Phase(P::Displeased) => Some(I::Displeased),
            C::Phase(P::Angered) => Some(I::Angered),
            C::AppeasedTemporarily { .. } => Some(I::Appeased { permanent: false }),
            C::AppeasedPermanently => Some(I::Appeased { permanent: true }),
        }
    }

    pub fn is_phase(&self, phase: GrandmapocalypsePhase) -> bool {
        self.phase().is_some_and(|p| p == phase)
    }

    pub fn is_appeased(&self) -> bool {
        matches!(
            self.current,
            Current::AppeasedTemporarily { .. } | Current::AppeasedPermanently
        )
    }

    pub fn is_appeased_temporarily(&self) -> bool {
        matches!(self.current, Current::AppeasedTemporarily { .. })
    }

    // TODO: This should affect CPS (0.95x).
    pub fn is_appeased_permanently(&self) -> bool {
        matches!(self.current, Current::AppeasedPermanently)
    }

    pub(crate) fn appeased_temporarily_times(&self) -> usize {
        self.appeased_temporarily_times
    }

    pub(crate) fn cps_mults(&self) -> &[f64] {
        &self.cps_mults
    }

    pub(crate) fn advance_phase(&mut self) {
        match &self.current {
            Current::None => self.current = Current::Phase(GrandmapocalypsePhase::Awoken),
            Current::Phase(GrandmapocalypsePhase::Awoken) => {
                self.current = Current::Phase(GrandmapocalypsePhase::Displeased)
            }
            Current::Phase(GrandmapocalypsePhase::Displeased) => {
                self.current = Current::Phase(GrandmapocalypsePhase::Angered)
            }
            _ => {}
        }
    }

    pub(crate) fn appease_temporarily(&mut self) {
        self.current = Current::AppeasedTemporarily {
            refresh: Refresh::new(self.appeased_duration),
        }
    }

    pub(crate) fn appease_permanently(&mut self) {
        self.current = Current::AppeasedPermanently;
    }

    pub(crate) fn unappease(&mut self) {
        if self.is_appeased() {
            self.current = Current::Phase(GrandmapocalypsePhase::Angered);
        }
    }

    pub(crate) fn modify_appeased_duration(&mut self, mut f: impl FnMut(&mut f64)) {
        f(&mut self.appeased_duration);

        if let Current::AppeasedTemporarily { refresh, .. } = &mut self.current {
            refresh.modify(f);
        }
    }

    pub(crate) fn add_cps_mult(&mut self, mult: f64) {
        self.cps_mults.push(mult);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Current {
    None,
    Phase(GrandmapocalypsePhase),
    AppeasedTemporarily { refresh: Refresh },
    AppeasedPermanently,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrandmapocalypsePhase {
    Awoken,
    Displeased,
    Angered,
}

pub enum GrandmapocalypseInfo {
    Awoken,
    Displeased,
    Angered,
    Appeased { permanent: bool },
}
