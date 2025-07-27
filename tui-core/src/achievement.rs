use crate::{
    Computed, State, macros,
    req::{Cmp, LateReq},
};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Serialize};
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

macros::serialize_via_state!(Achievements => BTreeSet<Achievement> as |a| a.owned);
macros::deserialize_via_state!(Achievements => BTreeSet<Achievement> as Achievements::from_owned);

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
    /* -------------------------------------------------------------------------- */
    /*                                Cookies Baked                               */
    /* -------------------------------------------------------------------------- */
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
    /* -------------------------------------------------------------------------- */
    /*                             Cookies Per Second                             */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::Cps(1.0))]
    CasualBaking,
    #[assoc(req = AchievementReq::Cps(10.0))]
    HardcoreBaking,
    #[assoc(req = AchievementReq::Cps(100.0))]
    SteadyTastyStream,
    #[assoc(req = AchievementReq::Cps(1.0 * num::THOUSAND))]
    CookieMonster,
    #[assoc(req = AchievementReq::Cps(10.0 * num::THOUSAND))]
    MassProducer,
    #[assoc(req = AchievementReq::Cps(1.0 * num::MILLION))]
    CookieVortex,
    #[assoc(req = AchievementReq::Cps(10.0 * num::MILLION))]
    CookiePulsar,
    #[assoc(req = AchievementReq::Cps(100.0 * num::MILLION))]
    CookieQuasar,
    #[assoc(req = AchievementReq::Cps(1.0 * num::BILLION))]
    #[name(base = "Oh Hey You're Still Here")]
    OhHeyYoureStillHere,
    #[assoc(req = AchievementReq::Cps(10.0 * num::BILLION))]
    #[name(base = "Let's Never Bake Again")]
    LetsNeverBakeAgain,
    #[assoc(req = AchievementReq::Cps(1.0 * num::TRILLION))]
    AWorldFilledWithCookies,
    #[assoc(req = AchievementReq::Cps(10.0 * num::TRILLION))]
    WhenThisBabyHits36QuadrillionCookiesPerHour,
    #[assoc(req = AchievementReq::Cps(100.0 * num::TRILLION))]
    FastAndDelicious,
    #[assoc(req = AchievementReq::Cps(1.0 * num::QUADRILLION))]
    CookieHertzAReallyReallyTastyHertz,
    #[assoc(req = AchievementReq::Cps(10.0 * num::QUADRILLION))]
    #[name(base = "Whoops, You Solved World Hunger")]
    WhoopsYouSolvedWorldHunger,
    #[assoc(req = AchievementReq::Cps(1.0 * num::QUINTILLION))]
    Turbopuns,
    #[assoc(req = AchievementReq::Cps(10.0 * num::QUINTILLION))]
    FasterMenner,
    #[assoc(req = AchievementReq::Cps(100.0 * num::QUINTILLION))]
    #[name(base = "And Yet You're Still Hungry")]
    AndYetYoureStillHungry,
    #[assoc(req = AchievementReq::Cps(1.0 * num::SEXTILLION))]
    TheAbakening,
    #[assoc(req = AchievementReq::Cps(10.0 * num::SEXTILLION))]
    #[name(base = "There's A Hard Limit To How Long These Achievement Names Can Be")]
    TheresAHardLimitToHowLongTheseAchievementNamesCanBe,
    #[assoc(req = AchievementReq::Cps(1.0 * num::SEPTILLION))]
    Fast,
    #[assoc(req = AchievementReq::Cps(10.0 * num::SEPTILLION))]
    KneadForSpeed,
    #[assoc(req = AchievementReq::Cps(100.0 * num::SEPTILLION))]
    #[name(base = "Well The Cookies Start Coming And They Don't Stop Coming")]
    WellTheCookiesStartComingAndTheyDontStopComing,
    #[assoc(req = AchievementReq::Cps(1.0 * num::OCTILLION))]
    #[name(base = "I Don't Know If You've Noticed But This Text Overflows")]
    IDontKnowIfYouveNoticedButThisTextOverflows,
    #[assoc(req = AchievementReq::Cps(10.0 * num::OCTILLION))]
    TheProofOfTheCookieIsInTheBaking,
    #[assoc(req = AchievementReq::Cps(1.0 * num::NONILLION))]
    #[name(base = "If It's Worth Doing It's Worth Overdoing")]
    IfItsWorthDoingItsWorthOverdoing,
    #[assoc(req = AchievementReq::Cps(10.0 * num::NONILLION))]
    RunningWithScissors,
    #[assoc(req = AchievementReq::Cps(100.0 * num::NONILLION))]
    RarefiedAir,
    #[assoc(req = AchievementReq::Cps(1.0 * num::DECILLION))]
    PushItToTheLimit,
    #[assoc(req = AchievementReq::Cps(10.0 * num::DECILLION))]
    GreenCookiesSleepFuriously,
    #[assoc(req = AchievementReq::Cps(1.0 * num::UNDECILLION))]
    LeisurelyPace,
    #[assoc(req = AchievementReq::Cps(10.0 * num::UNDECILLION))]
    Hypersonic,
    #[assoc(req = AchievementReq::Cps(100.0 * num::UNDECILLION))]
    GottaGoFast,
    #[assoc(req = AchievementReq::Cps(1.0 * num::DUODECILLION))]
    #[name(base = "Bake Him Away, Toys")]
    BakeHimAwayToys,
    #[assoc(req = AchievementReq::Cps(10.0 * num::DUODECILLION))]
    #[name(base = "You're #1 So Why Try Harder")]
    YoureNumber1SoWhyTryHarder,
    #[assoc(req = AchievementReq::Cps(1.0 * num::TREDECILLION))]
    #[name(base = "Haven't Even Begun To Peak")]
    HaventEvenBegunToPeak,
    #[assoc(req = AchievementReq::Cps(10.0 * num::TREDECILLION))]
    WhatDidWeEvenEatBeforeThese,
    #[assoc(req = AchievementReq::Cps(100.0 * num::TREDECILLION))]
    HeavyFlow,
    #[assoc(req = AchievementReq::Cps(1.0 * num::QUATTORDECILLION))]
    #[name(base = "More You Say?")]
    MoreYouSay,
    #[assoc(req = AchievementReq::Cps(10.0 * num::QUATTORDECILLION))]
    KeepGoingUntilISayStop,
    #[assoc(req = AchievementReq::Cps(1.0 * num::QUINDECILLION))]
    #[name(base = "But I Didn't Say Stop, Did I?")]
    ButIDidntSayStopDidI,
    #[assoc(req = AchievementReq::Cps(10.0 * num::QUINDECILLION))]
    WithUnrivaledFervor,
    #[assoc(req = AchievementReq::Cps(100.0 * num::QUINDECILLION))]
    IAmSpeed,
    #[assoc(req = AchievementReq::Cps(1.0 * num::SEXDECILLION))]
    AndOnAndOn,
    #[assoc(req = AchievementReq::Cps(10.0 * num::SEXDECILLION))]
    EverythingHappensSoMuch,
    #[assoc(req = AchievementReq::Cps(1.0 * num::SEPTENDECILLION))]
    #[name(base = "I'll Rest When I'm Dead")]
    IllRestWhenImDead,
    #[assoc(req = AchievementReq::Cps(10.0 * num::SEPTENDECILLION))]
    CanWeGetMuchHigher,
    #[assoc(req = AchievementReq::Cps(100.0 * num::SEPTENDECILLION))]
    #[name(base = "Speed's The Name Of The Game")]
    SpeedsTheNameOfTheGame,
    /* -------------------------------------------------------------------------- */
    /*                               Golden Cookies                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::GoldenCookieClickedCount(1))]
    GoldenCookie,
    #[assoc(req = AchievementReq::GoldenCookieClickedCount(7))]
    LuckyCookie,
    #[assoc(req = AchievementReq::GoldenCookieClickedCount(27))]
    AStrokeOfLuck,
    #[assoc(req = AchievementReq::GoldenCookieClickedCount(77))]
    Fortune,
    #[assoc(req = AchievementReq::GoldenCookieClickedCount(777))]
    Leprechaun,
    #[assoc(req = AchievementReq::GoldenCookieClickedCount(7_777))]
    BlackCatsPaw,
    #[assoc(req = AchievementReq::GoldenCookieClickedAtMost1sAfterSpawn)]
    EarlyBird,
    #[assoc(req = AchievementReq::GoldenCookieClickedAtMost1sBeforeDespawn)]
    FadingLuck,
    // TODO: Add other achievements for grandmas and all the other shit.
    #[assoc(req = AchievementReq::GrandmaJobCount(7))]
    Elder,
}

pub enum AchievementReq {
    CookiesBaked(f64),
    Cps(f64),
    GrandmaJobCount(u16),
    GoldenCookieClickedCount(usize),
    GoldenCookieClickedAtMost1sAfterSpawn,
    GoldenCookieClickedAtMost1sBeforeDespawn,
}

impl AchievementReq {
    fn as_late_req(&self) -> LateReq {
        match self {
            Self::CookiesBaked(v) => LateReq::CookiesAllTime(Cmp::AboveOrEq(*v)),
            Self::Cps(v) => LateReq::Cps(Cmp::AboveOrEq(*v)),
            Self::GrandmaJobCount(v) => LateReq::GrandmaJobUpgradeCount(Cmp::AboveOrEq(*v)),
            Self::GoldenCookieClickedCount(v) => LateReq::GoldenCookieClicked(Cmp::AboveOrEq(*v)),
            Self::GoldenCookieClickedAtMost1sAfterSpawn => {
                LateReq::GoldenCookieClickedAtMost1sAfterSpawn()
            }
            Self::GoldenCookieClickedAtMost1sBeforeDespawn => {
                LateReq::GoldenCookieClickedAtMost1sBeforeDespawn()
            }
        }
    }
}
