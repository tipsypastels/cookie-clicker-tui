use crate::{Achievement, calc};
use approx_eq_trait::ApproxEq;
use cookie_clicker_tui_utils::frames::RefreshClock;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Milk {
    state: MilkState,
    ratio: f64,
    cps_mult: f64,
    flavor: MilkFlavor,
    refresh: RefreshClock<15>,
}

impl Milk {
    pub(crate) fn new() -> Self {
        Self::from_state(Default::default())
    }

    fn from_state(state: MilkState) -> Self {
        let ratio = state.achievements as f64 / Achievement::VARIANT_COUNT as f64;
        let cps = calc::kitten_cps_mult(ratio, &state.kitten_mults);
        let flavor = MilkFlavor::find(state.achievements);
        let refresh = RefreshClock::new();

        Self {
            state,
            ratio,
            cps_mult: cps,
            flavor,
            refresh,
        }
    }

    pub(crate) fn tick(&mut self, achievements: u16) {
        if self.refresh.finish() {
            if achievements > self.state.achievements {
                self.state.achievements = achievements;
                self.ratio = achievements as f64 / Achievement::VARIANT_COUNT as f64;
                self.cps_mult = calc::kitten_cps_mult(self.ratio, &self.state.kitten_mults);
                self.flavor = MilkFlavor::find(achievements);
            };
            self.refresh.restart();
        }
    }

    pub(crate) fn add_kitten_mult(&mut self, mult: f64) {
        self.state.kitten_mults.push(mult);
        self.cps_mult = calc::kitten_cps_mult(self.ratio, &self.state.kitten_mults);
    }

    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    pub fn cps_mult(&self) -> f64 {
        self.cps_mult
    }

    pub fn is_empty(&self) -> bool {
        self.ratio.approx_eq(0.0)
    }

    pub fn flavor(&self) -> MilkFlavor {
        self.flavor
    }
}

impl Serialize for Milk {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.state.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Milk {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        MilkState::deserialize(de).map(Self::from_state)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct MilkState {
    achievements: u16,
    kitten_mults: Vec<f64>,
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
    fn find(len: u16) -> Self {
        let total = Achievement::VARIANT_COUNT as f64;
        let ratio = len as f64 / total;

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
