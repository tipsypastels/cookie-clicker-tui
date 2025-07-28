use cookie_clicker_tui_utils::refresh::Refresh;
use serde::{Deserialize, Serialize};

const DEFAULT_APPEASED_DURATION_SECS: f64 = 30.0 * 60.0;

#[derive(Serialize, Deserialize, Debug)]
pub struct Grandmapocalypse {
    current: Current,
    appeased_times: usize,
    appeased_duration: f64,
    appease_is_permanent: bool,
    cps_mults: Vec<f64>,
}

impl Grandmapocalypse {
    pub(crate) fn new() -> Self {
        Self {
            current: Current::None,
            appeased_times: 0,
            appeased_duration: DEFAULT_APPEASED_DURATION_SECS,
            appease_is_permanent: false,
            cps_mults: Vec::new(),
        }
    }

    pub(crate) fn tick(&mut self) {
        if let Current::AppeasedTemporarily { return_to, refresh } = &mut self.current
            && refresh.finish()
        {
            self.current = Current::Phase(*return_to);
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
            C::AppeasedPermanently { .. } => Some(I::Appeased { permanent: true }),
        }
    }

    pub fn is_phase(&self, phase: GrandmapocalypsePhase) -> bool {
        self.phase().is_some_and(|p| p == phase)
    }

    pub fn is_appeased(&self) -> bool {
        matches!(
            self.current,
            Current::AppeasedTemporarily { .. } | Current::AppeasedPermanently { .. }
        )
    }

    pub(crate) fn appeased_times(&self) -> usize {
        self.appeased_times
    }

    pub(crate) fn appeased_duration(&self) -> f64 {
        self.appeased_duration
    }

    pub(crate) fn appease_is_permanent(&self) -> bool {
        self.appease_is_permanent
    }

    pub(crate) fn cps_mults(&self) -> &[f64] {
        &self.cps_mults
    }

    pub(crate) fn set_phase(&mut self, phase: GrandmapocalypsePhase) {
        self.current = Current::Phase(phase);
    }

    pub(crate) fn appease(&mut self) {
        let Some(phase) = self.phase() else {
            return;
        };
        if self.appease_is_permanent {
            self._appease_permanently(phase);
        } else {
            self._appease_temporarily(phase);
        }
    }

    pub(crate) fn modify_appeased_duration(&mut self, mut f: impl FnMut(&mut f64)) {
        f(&mut self.appeased_duration);

        if let Current::AppeasedTemporarily { refresh, .. } = &mut self.current {
            refresh.modify(f);
        }
    }

    pub(crate) fn set_appease_permanently(&mut self, enable: bool) {
        self.appease_is_permanent = enable;

        match &self.current {
            Current::AppeasedTemporarily { return_to, .. } if enable => {
                self._appease_permanently(*return_to);
            }
            Current::AppeasedPermanently { return_to } if !enable => {
                self._appease_temporarily(*return_to);
            }
            _ => {}
        }
    }

    pub(crate) fn add_cps_mult(&mut self, mult: f64) {
        self.cps_mults.push(mult);
    }

    fn _appease_temporarily(&mut self, phase: GrandmapocalypsePhase) {
        self.current = Current::AppeasedTemporarily {
            return_to: phase,
            refresh: Refresh::new(self.appeased_duration),
        };
    }

    fn _appease_permanently(&mut self, phase: GrandmapocalypsePhase) {
        self.current = Current::AppeasedPermanently { return_to: phase };
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Current {
    None,
    Phase(GrandmapocalypsePhase),
    AppeasedTemporarily {
        return_to: GrandmapocalypsePhase,
        refresh: Refresh,
    },
    AppeasedPermanently {
        return_to: GrandmapocalypsePhase,
    },
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
