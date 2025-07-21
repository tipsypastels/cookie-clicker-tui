use crate::Achievement;
use approx_eq_trait::ApproxEq;
use cookie_clicker_tui_utils::frames::RefreshClock;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Milk {
    achievements: u16,
    ratio: f64,
    flavor: MilkFlavor,
    refresh: RefreshClock<15>,
}

impl Milk {
    pub(crate) fn new() -> Self {
        Self::_new(0)
    }

    fn _new(achievements: u16) -> Self {
        let flavor = MilkFlavor::find(achievements);
        let ratio = achievements as f64 / Achievement::VARIANT_COUNT as f64;
        let refresh = RefreshClock::new();

        Self {
            achievements,
            ratio,
            flavor,
            refresh,
        }
    }

    pub(crate) fn tick(&mut self, achievements: u16) {
        if self.refresh.finish() {
            if achievements > self.achievements {
                *self = Self::_new(achievements);
            } else {
                self.refresh.restart();
            }
        }
    }

    pub fn ratio(&self) -> f64 {
        self.ratio
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
        self.achievements.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Milk {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        u16::deserialize(de).map(Self::_new)
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
