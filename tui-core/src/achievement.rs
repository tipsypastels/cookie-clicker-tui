use crate::{
    Building, Computed, State, macros,
    req::{Cmp, LateReq},
};
use cookie_clicker_tui_utils::{num, refresh::Refresh};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, VecDeque};

// can't be a method since we need to access &state
// but also &mut Achievements, which is a field of state
pub fn tick(state: &mut State, computed: &Computed) {
    if state.achievements.refresh.finish() {
        state.achievements.refresh.reset();
        state.achievements.display_queue.pop_front();

        for achievement in Achievement::variants() {
            if state.achievements.owned.contains(&achievement) {
                continue;
            }

            if achievement.req().into_late_req().check(state, computed) {
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
    refresh: Refresh,
}

impl Achievements {
    pub fn new() -> Self {
        Self::from_owned(BTreeSet::new())
    }

    fn from_owned(owned: BTreeSet<Achievement>) -> Self {
        Self {
            owned,
            display_queue: VecDeque::new(),
            refresh: Refresh::new(10.0),
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
    /*                                  Clicking                                  */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(1.0 * num::THOUSAND))]
    Clicktastic,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(100.0 * num::THOUSAND))]
    Clickathon,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(10.0 * num::MILLION))]
    Clickolympics,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(1.0 * num::BILLION))]
    Clickorama,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(100.0 * num::BILLION))]
    Clickasmic,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(10.0 * num::TRILLION))]
    Clickageddon,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(1.0 * num::QUADRILLION))]
    Clicknarok,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(100.0 * num::QUADRILLION))]
    Clickastrophe,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(10.0 * num::QUINTILLION))]
    Clickataclysm,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(1.0 * num::SEXTILLION))]
    TheUltimateClickdown,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(100.0 * num::SEXTILLION))]
    AllTheOtherKidsWithThePumpedUpClicks,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(10.0 * num::SEPTILLION))]
    #[name(base = "One... More... Click...")]
    OneMoreClick,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(1.0 * num::OCTILLION))]
    ClicketySplit,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(100.0 * num::OCTILLION))]
    #[name(base = "Ain't That A Click In The Head")]
    AintThatAClickInTheHead,
    #[assoc(req = AchievementReq::CookiesBakedFromClicking(10.0 * num::NONILLION))]
    #[name(base = "What's Not Clicking")]
    WhatsNotClicking,
    /* -------------------------------------------------------------------------- */
    /*                              Building: Cursor                              */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 1))]
    Click,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 2))]
    #[name(base = "Double-Click")]
    DoubleClick,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 50))]
    MouseWheel,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 100))]
    OfMiceAndMen,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 200))]
    TheDigital,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 300))]
    ExtremePolydactyly,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 400))]
    #[name(base = "Dr. T")]
    DrT,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 500))]
    #[name(base = "Thumbs, Phalanges, Metacarpals")]
    ThumbsPhalangesMetacarpals,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 600))]
    WithHerFingerAndHerThumb,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 700))]
    GottaHandItToYou,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 800))]
    #[name(base = "The Devil's Workshop")]
    TheDevilsWorkshop,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 900))]
    AllOnDeck,
    #[assoc(req = AchievementReq::BuildingCount(Building::Cursor, 1000))]
    ARoundOfApplause,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Cursor, 10.0 * num::QUINTILLION))]
    ClickDelegator,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Cursor, 100.0 * num::SEPTILLION))]
    #[name(base = "Finger Clickin' Good")]
    FingerClickinGood,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Cursor, 1.0 * num::DECILLION))]
    #[name(base = "Click (Starring Adam Sandler)")]
    ClickStarringAdamSandler,
    /* -------------------------------------------------------------------------- */
    /*                              Building: Grandma                             */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::SellAGrandma)]
    JustWrong,
    #[assoc(req = AchievementReq::GrandmaJobCount(7))]
    Elder,
    #[assoc(req = AchievementReq::GrandmaJobCount(14))]
    Veteran,
    #[assoc(req = AchievementReq::BuildingCombinedCount(Building::Cursor, Building::Grandma, 777))]
    TheElderScrolls,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 1))]
    #[name(base = "Grandma's Cookies")]
    GrandmasCookies,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 50))]
    SloppyKisses,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 100))]
    RetirementHome,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 150))]
    FriendOfTheAncients,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 200))]
    RulerOfTheAncients,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 250))]
    TheOldNeverBotheredMeAnyway,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 300))]
    TheAgemaster,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 350))]
    ToOldlyGo,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 400))]
    AgedWell,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 450))]
    OneHundredAndFirstBirthday,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 500))]
    #[name(base = "But Wait 'Til You Get Older")]
    ButWaitTilYouGetOlder,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 550))]
    DefenseOfTheAncients,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 600))]
    OkayBoomer,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 650))]
    TheyMoistlyComeAtNight,
    #[assoc(req = AchievementReq::BuildingCount(Building::Grandma, 700))]
    #[name(base = "And Now You're Even Older")]
    AndNowYoureEvenOlder,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Grandma, 10.0 * num::QUINTILLION))]
    GushingGrannies,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Grandma, 100.0 * num::SEPTILLION))]
    PanicAtTheBingo,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Grandma, 1.0 * num::DECILLION))]
    Frantiquities,
    /* -------------------------------------------------------------------------- */
    /*                               Building: Farm                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 1))]
    BoughtTheFarm,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 50))]
    ReapWhatYouSow,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 100))]
    FarmIll,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 150))]
    PerfectedAgriculture,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 200))]
    Homegrown,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 250))]
    GardenerExtraordinaire,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 300))]
    SeedyBusiness,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 350))]
    #[name(base = "You, And The Beanstalk")]
    YouAndTheBeanstalk,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 400))]
    HarvestMoon,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 450))]
    MakeLikeATree,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 500))]
    SharpestToolInTheShed,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 550))]
    Overripe,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 600))]
    InTheGreen,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 650))]
    #[name(base = "It's Grown On You")]
    ItsGrownOnYou,
    #[assoc(req = AchievementReq::BuildingCount(Building::Farm, 700))]
    AuNaturel,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Farm, 100.0 * num::TRILLION))]
    IHateManure,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Farm, 1.0 * num::SEXTILLION))]
    RakeInTheDough,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Farm, 10.0 * num::OCTILLION))]
    Overgrowth,
    /* -------------------------------------------------------------------------- */
    /*                               Building: Mine                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 1))]
    YouKnowTheDrill,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 50))]
    ExcavationSite,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 100))]
    HollowThePlanet,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 150))]
    CanYouDigIt,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 200))]
    CenterOfTheEarth,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 250))]
    TectonicAmbassador,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 300))]
    FreakFracking,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 350))]
    RomancingTheStone,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 400))]
    #[name(base = "Mine?")]
    Mine,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 450))]
    CaveStory,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 500))]
    #[name(base = "Hey Now, You're A Rock")]
    HeyNowYoureARock,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 550))]
    RockOn,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 600))]
    #[name(base = "Mountain Out Of A Molehill, But Like In A Good Way")]
    MountainOutOfAMolehillButLikeInAGoodWay,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 650))]
    #[name(base = "Don't Let The Walls Cave In On You")]
    DontLetTheWallsCaveInOnYou,
    #[assoc(req = AchievementReq::BuildingCount(Building::Mine, 700))]
    #[name(base = "Dirt-Rich")]
    DirtRich,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Mine, 1.0 * num::QUADRILLION))]
    NeverDigDown,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Mine, 10.0 * num::SEXTILLION))]
    QuarryOn,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Mine, 100.0 * num::OCTILLION))]
    Sedementalism,
    /* -------------------------------------------------------------------------- */
    /*                              Building: Factory                             */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 1))]
    ProductionChain,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 50))]
    IndustrialRevolution,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 100))]
    GlobalWarming,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 150))]
    UltimateAutomation,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 200))]
    Technocracy,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 250))]
    RiseOfTheMachines,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 300))]
    ModernTimes,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 350))]
    ExMachina,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 400))]
    InFullGear,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 450))]
    #[name(base = "In-Cog-Neato")]
    InCogNeato,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 500))]
    BreakTheMold,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 550))]
    #[name(base = "Self-Manmade Man")]
    SelfManmadeMan,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 600))]
    TheWheelsOfProgress,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 650))]
    ReplacedByRobots,
    #[assoc(req = AchievementReq::BuildingCount(Building::Factory, 700))]
    BotsBuildBots,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Factory, 10.0 * num::QUADRILLION))]
    TheIncredibleMachine,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Factory, 100.0 * num::SEXTILLION))]
    YesILoveTechnology,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Factory, 1.0 * num::NONILLION))]
    LaborOfLove,
    /* -------------------------------------------------------------------------- */
    /*                               Building: Bank                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 1))]
    PrettyPenny,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 50))]
    FitTheBill,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 100))]
    ALoanInTheDark,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 150))]
    NeedForGreed,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 200))]
    #[name(base = "It's The Economy, Stupid")]
    ItsTheEconomyStupid,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 250))]
    AcquireCurrency,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 300))]
    TheNerveOfWar,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 350))]
    AndINeedItNow,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 400))]
    TreacleTartEconomics,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 450))]
    #[name(base = "Save Your Breath Because That's All You've Got Left")]
    SaveYourBreathBecauseThatsAllYouveGotLeft,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 500))]
    #[name(base = "Get The Show On, Get Paid")]
    GetTheShowOnGetPaid,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 550))]
    ChecksOut,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 600))]
    #[name(base = "That's Rich")]
    ThatsRich,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 650))]
    FinancialProdigy,
    #[assoc(req = AchievementReq::BuildingCount(Building::Bank, 700))]
    GettingThatBag,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Bank, 100.0 * num::QUADRILLION))]
    VestedInterest,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Bank, 1.0 * num::SEPTILLION))]
    PaidInFull,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Bank, 10.0 * num::NONILLION))]
    ReverseFunnelSystem,
    /* -------------------------------------------------------------------------- */
    /*                              Building: Temple                              */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 1))]
    YourTimeToShine,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 50))]
    ShadySect,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 100))]
    #[name(base = "New-Age Cult")]
    NewAgeCult,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 150))]
    OrganizedReligion,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 200))]
    Fanaticism,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 250))]
    Zealotry,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 300))]
    Wololo,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 350))]
    PrayOnTheWeak,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 400))]
    #[name(base = "Holy Cookies, Grandma!")]
    HolyCookiesGrandma,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 450))]
    VengefulAndAlmighty,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 500))]
    #[name(base = "My World's On Fire, How About Yours")]
    MyWorldsOnFireHowAboutYours,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 550))]
    LivingOnAPrayer,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 600))]
    PreachesAndCream,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 650))]
    AndIWillPrayToABigGod,
    #[assoc(req = AchievementReq::BuildingCount(Building::Temple, 700))]
    #[name(base = "The Leader Is Good, The Leader Is Great")]
    TheLeaderIsGoodTheLeaderIsGreat,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Temple, 1.0 * num::QUINTILLION))]
    NewWorldOrder,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Temple, 10.0 * num::SEPTILLION))]
    ChurchOfCookiology,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::Temple, 100.0 * num::NONILLION))]
    ThusSpokeYou,
    /* -------------------------------------------------------------------------- */
    /*                           Building: Wizard Tower                           */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 1))]
    Bewitched,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 50))]
    #[name(base = "The Sorcerer's Apprentice")]
    TheSorcerersApprentice,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 100))]
    CharmsAndEnchantments,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 150))]
    CursesAndMaledictions,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 200))]
    MagicKingdom,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 250))]
    TheWitchingWorld,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 300))]
    AndNowForMyNextTrick,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 350))]
    #[name(base = "It's A Kind of Magic")]
    ItsAKindOfMagic,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 400))]
    ThePrestige,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 450))]
    SpellItOutForYou,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 500))]
    TheMeteorMenBegToDiffer,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 550))]
    HigitusFigitusMigitusMum,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 600))]
    MagicThinking,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 650))]
    ShospleColupis,
    #[assoc(req = AchievementReq::BuildingCount(Building::WizardTower, 700))]
    #[name(base = "You Don't Think They Could've Used...")]
    YouDontThinkTheyCouldveUsed,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::WizardTower, 10.0 * num::QUINTILLION))]
    HocusPocus,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::WizardTower, 100.0 * num::SEPTILLION))]
    #[name(base = "Too Many Rabbits, Not Enough Hats")]
    TooManyRabbitsNotEnoughHats,
    #[assoc(req = AchievementReq::BuildingCookiesBaked(Building::WizardTower, 1.0 * num::DECILLION))]
    ManafestDestiny,
    // TODO: The rest.
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
}

pub enum AchievementReq {
    CookiesBaked(f64),
    CookiesBakedFromClicking(f64),
    BuildingCount(Building, u16),
    BuildingCombinedCount(Building, Building, u16),
    BuildingCookiesBaked(Building, f64),
    Cps(f64),
    GrandmaJobCount(u16),
    GoldenCookieClickedCount(usize),
    GoldenCookieClickedAtMost1sAfterSpawn,
    GoldenCookieClickedAtMost1sBeforeDespawn,
    SellAGrandma,
}

impl AchievementReq {
    fn into_late_req(self) -> LateReq {
        match self {
            Self::CookiesBaked(v) => LateReq::CookiesAllTime(Cmp::AboveOrEq(v)),
            Self::CookiesBakedFromClicking(v) => {
                LateReq::CookiesAllTimeFromClicking(Cmp::AboveOrEq(v))
            }
            Self::BuildingCount(b, v) => LateReq::BuildingCount(b, Cmp::AboveOrEq(v)),
            Self::BuildingCombinedCount(b1, b2, v) => {
                LateReq::CustomBox(Box::new(move |state, _| {
                    state.buildings.count(b1) + state.buildings.count(b2) >= v
                }))
            }
            Self::BuildingCookiesBaked(b, v) => {
                LateReq::BuildingCookiesAllTime(b, Cmp::AboveOrEq(v))
            }
            Self::Cps(v) => LateReq::Cps(Cmp::AboveOrEq(v)),
            Self::GrandmaJobCount(v) => LateReq::GrandmaJobUpgradeCount(Cmp::AboveOrEq(v)),
            Self::GoldenCookieClickedCount(v) => LateReq::GoldenCookieClicked(Cmp::AboveOrEq(v)),
            Self::GoldenCookieClickedAtMost1sAfterSpawn => {
                LateReq::GoldenCookieClickedAtMost1sAfterSpawn()
            }
            Self::GoldenCookieClickedAtMost1sBeforeDespawn => {
                LateReq::GoldenCookieClickedAtMost1sBeforeDespawn()
            }
            Self::SellAGrandma => LateReq::Custom(|state, _| state.buildings.grandma_been_sold()),
        }
    }
}
