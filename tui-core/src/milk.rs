use cookie_clicker_tui_utils::frames::RefreshClock;
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use std::ops::RangeInclusive;

const PERCENT_PER_ACHIEVEMENT: u16 = 4;

#[derive(Debug)]
pub struct Milk {
    achievements: u16,
    percent: u16,
    flavor: MilkFlavor,
    refresh: RefreshClock<15>,
}

impl Milk {
    pub(crate) fn new() -> Self {
        Self::_new(0)
    }

    fn _new(achievements: u16) -> Self {
        let percent = achievements * PERCENT_PER_ACHIEVEMENT;
        let flavor = MilkFlavor::find(achievements);
        let refresh = RefreshClock::new();

        Self {
            achievements,
            percent,
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

    pub fn percent(&self) -> u16 {
        self.percent
    }

    pub fn flavor(&self) -> MilkFlavor {
        self.flavor
    }
}

#[derive(Assoc, Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(fn achievement_range(&self) -> Option<RangeInclusive<u16>>)]
#[name(base = "title case")]
pub enum MilkFlavor {
    #[assoc(achievement_range = 0..=24)]
    Plain,
    #[assoc(achievement_range = 25..=49)]
    Chocolate,
    #[assoc(achievement_range = 50..=74)]
    Raspberry,
    #[assoc(achievement_range = 75..=99)]
    Orange,
    #[assoc(achievement_range = 100..=124)]
    Caramel,
    #[assoc(achievement_range = 125..=149)]
    Banana,
    #[assoc(achievement_range = 150..=174)]
    Lime,
    #[assoc(achievement_range = 175..=199)]
    Blueberry,
    #[assoc(achievement_range = 200..=224)]
    Strawberry,
    #[assoc(achievement_range = 225..=249)]
    Vanilla,
    #[assoc(achievement_range = 250..=274)]
    Honey,
    #[assoc(achievement_range = 275..=299)]
    Coffee,
    #[assoc(achievement_range = 300..=324)]
    Tea,
    #[assoc(achievement_range = 325..=349)]
    Coconut,
    #[assoc(achievement_range = 350..=374)]
    Cherry,
    #[assoc(achievement_range = 375..=399)]
    Spiced,
    #[assoc(achievement_range = 400..=424)]
    Maple,
    #[assoc(achievement_range = 425..=449)]
    Mint,
    #[assoc(achievement_range = 450..=474)]
    Licorice,
    #[assoc(achievement_range = 475..=499)]
    Rose,
    #[assoc(achievement_range = 500..=524)]
    Dragonfruit,
    #[assoc(achievement_range = 525..=549)]
    Melon,
    #[assoc(achievement_range = 550..=574)]
    Blackcurrant,
    #[assoc(achievement_range = 575..=599)]
    Peach,
    // fallback
    Hazelnut,
}

impl MilkFlavor {
    fn find(len: u16) -> Self {
        Self::variants().find(|f| f.matches(len)).unwrap()
    }

    fn matches(&self, len: u16) -> bool {
        let range = self.achievement_range();
        let range = range.as_ref();
        range.is_some_and(|r| r.contains(&len)) || range.is_none()
    }
}
