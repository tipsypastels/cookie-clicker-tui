use crate::{Achievement, calc, macros};
use cookie_clicker_tui_utils::frames::RefreshClock;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Serialize};

const PERCENT_PER_ACHIEVEMENT: u16 = 4;

#[derive(Debug)]
pub struct Milk {
    state: MilkState,
    computed: MilkComputed,
    refresh: RefreshClock<3>,
}

impl Milk {
    pub(crate) fn new() -> Self {
        Self::from_state(MilkState::default())
    }

    fn from_state(state: MilkState) -> Self {
        let computed = MilkComputed::new(&state);
        let refresh = RefreshClock::new();

        Self {
            state,
            computed,
            refresh,
        }
    }

    pub(crate) fn tick(&mut self, achievements: u16) {
        if self.refresh.finish() {
            if achievements > self.state.achievements {
                self.state.achievements = achievements;
                self.computed = MilkComputed::new(&self.state);
            }
            self.refresh.restart();
        }
    }

    pub fn percent(&self) -> u16 {
        self.computed.percent
    }

    pub fn ratio(&self) -> f64 {
        self.computed.ratio
    }

    pub fn cps_mult(&self) -> f64 {
        self.computed.cps_mult
    }

    pub fn is_empty(&self) -> bool {
        self.computed.percent == 0
    }

    pub fn flavor(&self) -> MilkFlavor {
        self.computed.flavor
    }

    pub(crate) fn add_kitten_factor(&mut self, factor: f64) {
        self.state.kitten_factors.push(factor);
        self.computed = MilkComputed::new(&self.state);
    }
}

macros::serialize_via_state!(Milk => MilkState as |m| m.state);
macros::deserialize_via_state!(Milk => MilkState as Milk::from_state);

#[derive(Serialize, Deserialize, Default, Debug)]
struct MilkState {
    achievements: u16,
    kitten_factors: Vec<f64>,
}

#[derive(Debug)]
struct MilkComputed {
    percent: u16,
    ratio: f64,
    cps_mult: f64,
    flavor: MilkFlavor,
}

impl MilkComputed {
    fn new(state: &MilkState) -> Self {
        let percent = state.achievements * PERCENT_PER_ACHIEVEMENT;
        let ratio = state.achievements as f64 / Achievement::VARIANT_COUNT as f64;
        let cps_mult = calc::kitten_cps_mult(percent, &state.kitten_factors);
        let flavor = MilkFlavor::find(ratio);

        Self {
            percent,
            ratio,
            cps_mult,
            flavor,
        }
    }
}

#[derive(Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[name(base = "title case")]
pub enum MilkFlavor {
    Plain,
    Chocolate,
    Raspberry,
    Orange,
    Caramel,
    Banana,
    Lime,
    Blueberry,
    Strawberry,
    Vanilla,
    Honey,
    Coffee,
    Tea,
    Coconut,
    Cherry,
    Spiced,
    Maple,
    Mint,
    Licorice,
    Rose,
    Dragonfruit,
    Melon,
    Blackcurrant,
    Peach,
    Hazelnut,
}

impl MilkFlavor {
    fn find(ratio: f64) -> Self {
        let total = Achievement::VARIANT_COUNT as f64;
        Self::variants()
            .enumerate()
            .zip(Self::variants().enumerate().skip(1))
            .find_map(|((cur_i, cur), (next_i, _))| {
                let cur_ratio = cur_i as f64 / total;
                let next_ratio = next_i as f64 / total;
                (ratio >= cur_ratio && ratio < next_ratio).then_some(cur)
            })
            .unwrap_or(Self::VARIANTS[Self::VARIANT_COUNT - 1])
    }
}
