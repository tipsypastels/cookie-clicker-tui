use crate::{Computed, State, req::LateReq};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeSet, VecDeque};

// can't be a method since we need to access &state
// but also &mut Achievements, which is a field of state
pub fn tick(state: &mut State, computed: &Computed) {
    if state.achievements.refresh.finish() {
        state.achievements.refresh.restart();
        state.achievements.display_queue.pop_front();

        for achievement in Achievement::variants() {
            if state.achievements.owned.contains(&achievement) {
                continue;
            }

            if achievement.req().as_late_req().check(state, computed) {
                state.achievements.owned.insert(achievement);
                state.achievements.display_queue.push_back(achievement);
            }
        }
    }
}

#[derive(Debug)]
pub struct Achievements {
    owned: BTreeSet<Achievement>,
    display_queue: VecDeque<Achievement>,
    refresh: RefreshClock<10>,
}

impl Achievements {
    pub fn new() -> Self {
        Self::from_owned(BTreeSet::new())
    }

    fn from_owned(owned: BTreeSet<Achievement>) -> Self {
        Self {
            owned,
            display_queue: VecDeque::new(),
            refresh: RefreshClock::new(),
        }
    }

    pub fn owned(&self) -> &BTreeSet<Achievement> {
        &self.owned
    }

    pub fn queued(&self) -> Option<Achievement> {
        self.display_queue.front().copied()
    }
}

impl Serialize for Achievements {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.owned.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Achievements {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        BTreeSet::deserialize(de).map(Self::from_owned)
    }
}

#[derive(
    Assoc,
    Name,
    Variants,
    Serialize,
    Deserialize,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[func(pub fn req(&self) -> AchievementReq)]
#[name(base = "title case")]
pub enum Achievement {
    #[assoc(req = AchievementReq::CookiesBaked(1.0))]
    WakeAndBake,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::THOUSAND))]
    MakingSomeDough,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::THOUSAND))]
    SoBakedRightNow,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::MILLION))]
    FledgelingBakery,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::MILLION))]
    AffluentBakery,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::BILLION))]
    #[name(base = "World-Famous Bakery")]
    WorldFamousBakery,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::BILLION))]
    CosmicBakery,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::TRILLION))]
    GalacticBaker,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::TRILLION))]
    UniversalBakery,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::QUADRILLION))]
    TimelessBakery,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::QUADRILLION))]
    InfiniteBakery,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::QUINTILLION))]
    ImmortalBakery,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::QUINTILLION))]
    #[name(base = "Don't Stop Me Now")]
    DontStopMeNow,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::SEXTILLION))]
    #[name(base = "You Can't Stop Me Now")]
    YouCanStopNow,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::SEXTILLION))]
    CookiesAllTheWayDown,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::SEPTILLION))]
    Overdose,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::SEPTILLION))]
    #[name(base = "How?")]
    How,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::OCTILLION))]
    TheLandOfMilkAndCookies,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::OCTILLION))]
    SheWhoControlsTheCookiesControlsTheUniverse,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::NONILLION))]
    TonightOnHoarders,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::NONILLION))]
    #[name(base = "Are You Gonna Eat All That?")]
    AreYouGonnaEatAllThat,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::DECILLION))]
    #[name(base = "We're Gonna Need A Bigger Bakery")]
    WereGonnaNeedABiggerBakery,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::DECILLION))]
    InTheMouthOfMadness,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::UNDECILLION))]
    #[name(base = "Brought To You By The Letter 🍪")]
    BroughtToYouByTheLetterCookie,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::UNDECILLION))]
    #[name(base = "The Dreams In Which I'm Baking Are The Best I've Ever Had")]
    TheDreamsInWhichImBakingAreTheBestIveEverHad,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::DUODECILLION))]
    SetForLife,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::DUODECILLION))]
    #[name(base = "Panic! At Nabisco")]
    PanicAtNabisco,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::TREDECILLION))]
    BurstingAtTheSeams,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::TREDECILLION))]
    JustAboutFull,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::QUATTORDECILLION))]
    HungryForMore,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::QUATTORDECILLION))]
    #[name(base = "Feed Me, Tipsy")]
    FeedMeTipsy,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::QUINDECILLION))]
    #[name(base = "And Then What?")]
    AndThenWhat,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::QUINDECILLION))]
    #[name(base = "I Think It's Safe To Say You've Got It made")]
    IThinkItsSafeToSayYouveGotItMade,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::SEXDECILLION))]
    ASometimesFood,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::SEXDECILLION))]
    NotEnoughOfAGoodThing,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::SEPTENDECILLION))]
    HornOfPlenty,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::SEPTENDECILLION))]
    LargeAndInCharge,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::OCTODECILLION))]
    AbsolutelyStuffed,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::OCTODECILLION))]
    #[name(base = "It's Only Wafer-Thin")]
    ItsOnlyWaferThin,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::NOVEMDECILLION))]
    ThinkBig,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::NOVEMDECILLION))]
    HypersizeMe,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::VIGINTILLION))]
    MaxCapacity,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::VIGINTILLION))]
    FakeItTillYouBakeIt,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::UNVIGINTILLION))]
    HistoryInTheBaking,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::UNVIGINTILLION))]
    WhatDoYouGetForTheBakerWhoHasEverything,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::DUOVIGINTILLION))]
    BottomlessPit,
    #[assoc(req = AchievementReq::CookiesBaked(100.0 * num::DUOVIGINTILLION))]
    RainyDayFund,
    #[assoc(req = AchievementReq::CookiesBaked(1.0 * num::TREVIGINTILLION))]
    AndALittleExtra,
}

#[derive(Debug)]
pub enum AchievementReq {
    CookiesBaked(f64),
}

impl AchievementReq {
    fn as_late_req(&self) -> LateReq {
        match self {
            Self::CookiesBaked(v) => LateReq::CookiesAllTimeAboveOrEq(*v),
        }
    }
}
