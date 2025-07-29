use super::effect_info::UpgradeEffectInfo;
use crate::{Cost, CostDyn, CostResolved, GrandmapocalypsePhase, State, calc, req::Req};
use cookie_clicker_tui_utils::num;

#[derive(Copy, Clone)]
pub enum Switch {
    ElderPledge,
    ElderCovenant,
    RevokeElderCovenant,
}

impl Switch {
    pub fn cost(&self) -> Cost {
        match self {
            Self::ElderPledge => Cost::Dyn(CostDyn::new(|state, _| {
                CostResolved::Cookies(calc::elder_pledge_cost(
                    state.grandmapocalypse.appeased_temporarily_times(),
                ))
            })),
            Self::ElderCovenant => Cost::Cookies(num::SIX_SIX_REPEATING * num::TRILLION),
            Self::RevokeElderCovenant => Cost::Cookies(num::SIX_REPEATING * num::BILLION),
        }
    }

    pub fn req(&self) -> Req {
        match self {
            Self::ElderPledge => Req::Custom(|state| {
                state
                    .grandmapocalypse
                    .is_phase(GrandmapocalypsePhase::Angered)
            }),
            Self::ElderCovenant => Req::Custom(|state| {
                !state.grandmapocalypse.is_appeased_permanently()
                    && state.grandmapocalypse.appeased_temporarily_times() > 0
                    && !state.grandmapocalypse.is_no_grandmas()
            }),
            Self::RevokeElderCovenant => {
                Req::Custom(|state| state.grandmapocalypse.is_appeased_permanently())
            }
        }
    }

    pub fn buy(&self, state: &mut State) {
        match self {
            Self::ElderPledge => {
                state
                    .grandmapocalypse
                    .appease_temporarily(&mut state.cookies);
            }
            Self::ElderCovenant => {
                state
                    .grandmapocalypse
                    .appease_permanently(&mut state.cookies);
            }
            Self::RevokeElderCovenant => {
                state.grandmapocalypse.unappease();
            }
        }
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        match self {
            Self::ElderPledge => UpgradeEffectInfo::ElderPledge,
            Self::ElderCovenant => UpgradeEffectInfo::ElderCovenant { revoke: false },
            Self::RevokeElderCovenant => UpgradeEffectInfo::ElderCovenant { revoke: true },
        }
    }
}
