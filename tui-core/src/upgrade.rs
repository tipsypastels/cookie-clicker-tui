mod click_and_cursor;
mod effect_info;
mod grandma_job;
mod kitten;
mod research;
mod switch;
mod tiered;

pub use effect_info::*;

use self::{
    click_and_cursor::{ClickAndCursor, ClickAndCursorMode},
    grandma_job::GrandmaJob,
    kitten::Kitten,
    research::Research,
    switch::Switch,
    tiered::Tiered,
};
use crate::{Building, Changeset, Cost, State, cps::Cps, req::Req};
use cookie_clicker_tui_utils::{num, refresh::Refresh};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, ops::Deref};

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnedUpgrades(BTreeSet<Upgrade>);

impl OwnedUpgrades {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn as_set(&self) -> &BTreeSet<Upgrade> {
        &self.0
    }

    pub fn has(&self, upgrade: Upgrade) -> bool {
        self.0.contains(&upgrade)
    }

    pub fn add(&mut self, upgrade: Upgrade) {
        self.0.insert(upgrade);
    }
}

#[derive(Debug)]
pub struct AvailableUpgrades {
    list: Box<[Upgrade]>,
    refresh: Refresh,
}

impl AvailableUpgrades {
    pub fn new(state: &State, cps: &Cps) -> Self {
        let mut v = Upgrade::variants()
            .filter(|u| !state.owned_upgrades.has(*u))
            .filter(|u| u.class().req().check(state))
            .collect::<Vec<_>>();

        v.sort_by(|a, b| Cost::total_cmp(a.cost(), b.cost(), state, cps));

        Self {
            list: v.into(),
            refresh: Refresh::new(5.0),
        }
    }

    pub fn tick(&mut self, state: &State, cps: &Cps, changeset: &Changeset) {
        if self.refresh.finish() || changeset.available_upgrades {
            *self = Self::new(state, cps);
        }
    }
}

impl Deref for AvailableUpgrades {
    type Target = [Upgrade];

    fn deref(&self) -> &Self::Target {
        &self.list
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
#[func(const fn class(self) -> UpgradeClass)]
#[name(base = "title case")]
#[name(extra(lower = "title case lower"))]
pub enum Upgrade {
    /* -------------------------------------------------------------------------- */
    /*                               Tiered: Cursor                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(0, ClickAndCursorMode::Double)))]
    ReinforcedIndexFinger,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(1, ClickAndCursorMode::Double)))]
    CarpalTunnelPreventionCream,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(2, ClickAndCursorMode::Double)))]
    Ambidextrous,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(3, ClickAndCursorMode::ThousandFingers)))]
    ThousandFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(4, ClickAndCursorMode::ThousandFingersMult(5.0))))]
    MillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(5, ClickAndCursorMode::ThousandFingersMult(10.0))))]
    BillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(6, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    TrillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(7, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    QuadrillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(8, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    QuintillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(9, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    SextillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(10, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    SeptillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(11, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    OctillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(12, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    NonillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(13, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    DecillionFingers,
    #[assoc(class = UpgradeClass::ClickAndCursor(ClickAndCursor::new(14, ClickAndCursorMode::ThousandFingersMult(20.0))))]
    UndecillionFingers,
    /* -------------------------------------------------------------------------- */
    /*                               Tiered: Grandma                              */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Grandma)))]
    ForwardsFromGrandma,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Grandma)))]
    #[name(base = "Steel-Plated Rolling Pins")]
    SteelPlatedRollingPins,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Grandma)))]
    LubricatedDentures,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Grandma)))]
    PruneJuice,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Grandma)))]
    #[name(base = "Double-Thick Glasses")]
    DoubleThickGlasses,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Grandma)))]
    AgingAgents,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Grandma)))]
    XtremeWalkers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Grandma)))]
    TheUnbridling,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Grandma)))]
    ReverseDementia,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Grandma)))]
    TimeproofHairDyes,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Grandma)))]
    GoodManners,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Grandma)))]
    GenerationDegeneration,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Grandma)))]
    Visits,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Grandma)))]
    KitchenCabinets,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Grandma)))]
    #[name(base = "Foam-Tipped Canes")]
    FoamTippedCanes,
    /* -------------------------------------------------------------------------- */
    /*                                Tiered: Farm                                */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Farm)))]
    CheapHoes,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Farm)))]
    Fertilizer,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Farm)))]
    CookieTrees,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Farm)))]
    #[name(base = "Genetically-Modified Cookies")]
    GeneticallyModifiedCookies,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Farm)))]
    GingerbreadScarecrows,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Farm)))]
    PulsarSprinklers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Farm)))]
    FudgeFungus,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Farm)))]
    WheatTriffids,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Farm)))]
    HumanePesticides,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Farm)))]
    Barnstars,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Farm)))]
    Lindworms,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Farm)))]
    GlobalSeedVault,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Farm)))]
    #[name(base = "Reverse-Veganism")]
    ReverseVeganism,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Farm)))]
    CookieMulch,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Farm)))]
    #[name(base = "Self-Driving Tractors")]
    SelfDrivingTractors,
    /* -------------------------------------------------------------------------- */
    /*                                Tiered: Mine                                */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Mine)))]
    SugarGas,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Mine)))]
    Megadrill,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Mine)))]
    Ultradrill,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Mine)))]
    Ultimadrill,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Mine)))]
    #[name(base = "H-Bomb Mining")]
    HBombMining,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Mine)))]
    Coreforge,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Mine)))]
    Planetsplitters,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Mine)))]
    CanolaOilWells,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Mine)))]
    MolePeople,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Mine)))]
    MineCanaries,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Mine)))]
    BoreAgain,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Mine)))]
    AirMining,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Mine)))]
    CaramelAlloys,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Mine)))]
    DeliciousMineralogy,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Mine)))]
    MineshaftSupports,
    /* -------------------------------------------------------------------------- */
    /*                               Tiered: Factory                              */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Factory)))]
    SturdierConveyorBelts,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Factory)))]
    ChildLabor,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Factory)))]
    Sweatshop,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Factory)))]
    RadiumReactors,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Factory)))]
    Recombobulators,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Factory)))]
    #[name(base = "Deep-Bake Process")]
    DeepBakeProcess,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Factory)))]
    CyborgWorkforce,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Factory)))]
    #[name(base = "78-Hour Days")]
    SeventyEightHourDays,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Factory)))]
    MachineLearning,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Factory)))]
    BrowniePointSystem,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Factory)))]
    #[name(base = "\"Volunteer\" Interns")]
    VolunteerInterns,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Factory)))]
    BehavioralReframing,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Factory)))]
    TheInfinityEngine,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Factory)))]
    #[name(base = "N-Dimensional Assembly Lines")]
    NDimensionalAssemblyLines,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Factory)))]
    UniversalAutomation,
    /* -------------------------------------------------------------------------- */
    /*                                Tiered: Bank                                */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Bank)))]
    TallerTellers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Bank)))]
    #[name(base = "Scissor-Resistant Credit Cards")]
    ScissorResistantCreditCards,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Bank)))]
    #[name(base = "Acid-Proof Vaults")]
    AcidProofVaults,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Bank)))]
    ChocolateCoins,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Bank)))]
    ExponentialInterestRates,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Bank)))]
    FinancialZen,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Bank)))]
    WayOfTheWallet,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Bank)))]
    TheStuffRationale,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Bank)))]
    EdibleMoney,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Bank)))]
    GrandSupercycle,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Bank)))]
    RulesOfAcquisition,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Bank)))]
    AltruisticLoop,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Bank)))]
    DiminishingTaxReturns,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Bank)))]
    CookiePoints,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Bank)))]
    TheBigShortcake,
    /* -------------------------------------------------------------------------- */
    /*                               Tiered: Temple                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Temple)))]
    GoldenIdols,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Temple)))]
    Sacrifices,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Temple)))]
    DeliciousBlessing,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Temple)))]
    SunFestival,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Temple)))]
    EnlargedPantheon,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Temple)))]
    GreatBakerInTheSky,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Temple)))]
    CreationMyth,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Temple)))]
    Theocracy,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Temple)))]
    SickRapPrayers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Temple)))]
    #[name(base = "Psalm-Reading")]
    PsalmReading,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Temple)))]
    WarOfTheGods,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Temple)))]
    ANovelIdea,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Temple)))]
    Apparitions,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Temple)))]
    Negatheism,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Temple)))]
    TempleTraps,
    /* -------------------------------------------------------------------------- */
    /*                            Tiered: Wizard Tower                            */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::WizardTower)))]
    PointierHats,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::WizardTower)))]
    BeardlierBeards,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::WizardTower)))]
    AncientGrimoires,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::WizardTower)))]
    KitchenCurses,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::WizardTower)))]
    SchoolOfSorcery,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::WizardTower)))]
    DarkFormulas,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::WizardTower)))]
    Cookiemancy,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::WizardTower)))]
    RabbitTrick,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::WizardTower)))]
    DeluxeTailoredWands,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::WizardTower)))]
    ImmobileSpellcasting,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::WizardTower)))]
    Electricity,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::WizardTower)))]
    SpellingBees,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::WizardTower)))]
    WizardBasements,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::WizardTower)))]
    MagicalRealism,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::WizardTower)))]
    Polymorphism,
    /* -------------------------------------------------------------------------- */
    /*                              Tiered: Shipment                              */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Shipment)))]
    VanillaNebulae,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Shipment)))]
    Wormholes,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Shipment)))]
    FrequentFlyer,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Shipment)))]
    WarpDrive,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Shipment)))]
    ChocolateMonoliths,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Shipment)))]
    GenerationShip,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Shipment)))]
    DysonSphere,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Shipment)))]
    TheFinalFrontier,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Shipment)))]
    Autopilot,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Shipment)))]
    RestaurantsAtTheEndOfTheUniverse,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Shipment)))]
    UniversalAlphabet,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Shipment)))]
    ToroidUniverse,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Shipment)))]
    PrimeDirective,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Shipment)))]
    CosmicForegroundRadiation,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Shipment)))]
    AtYourDoorstepIn30MinutesOrYourMoneyBack,
    /* -------------------------------------------------------------------------- */
    /*                             Tiered: Alchemy Lab                            */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::AlchemyLab)))]
    Antimony,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::AlchemyLab)))]
    EssenceOfDough,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::AlchemyLab)))]
    TrueChocolate,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::AlchemyLab)))]
    Ambrosia,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::AlchemyLab)))]
    AquaCrustulae,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::AlchemyLab)))]
    OriginCrucible,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::AlchemyLab)))]
    TheoryOfAtomicFluidity,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::AlchemyLab)))]
    BeigeGoo,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::AlchemyLab)))]
    TheAdventOfChemistry,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::AlchemyLab)))]
    OnSecondThought,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::AlchemyLab)))]
    PublicBetterment,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::AlchemyLab)))]
    HermeticReconciliation,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::AlchemyLab)))]
    ChromaticCycling,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::AlchemyLab)))]
    ArcanizedGlassware,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::AlchemyLab)))]
    TheDoseMakesThePoison,
    /* -------------------------------------------------------------------------- */
    /*                               Tiered: Portal                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Portal)))]
    AncientTablet,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Portal)))]
    InsaneOatlingWorkers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Portal)))]
    SoulBond,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Portal)))]
    SanityDance,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Portal)))]
    BraneTransplant,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Portal)))]
    #[name(base = "Deity-Sized Portals")]
    DeitySizedPortals,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Portal)))]
    #[name(base = "End Of Times Back-Up Plan")]
    EndOfTimesBackUpPlan,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Portal)))]
    MaddeningChants,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Portal)))]
    TheRealWorld,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Portal)))]
    DimensionalGarbageGulper,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Portal)))]
    EmbeddedMicroportals,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Portal)))]
    HisAdvent,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Portal)))]
    DomesticRifts,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Portal)))]
    PortalGuns,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Portal)))]
    AWayHome,
    /* -------------------------------------------------------------------------- */
    /*                            Tiered: Time Machine                            */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::TimeMachine)))]
    FluxCapacitors,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::TimeMachine)))]
    TimeParadoxResolver,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::TimeMachine)))]
    QuantumConundrum,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::TimeMachine)))]
    CausalityEnforcer,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::TimeMachine)))]
    YestermorrowComparators,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::TimeMachine)))]
    FarFutureEnactment,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::TimeMachine)))]
    GreatLoopHypothesis,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::TimeMachine)))]
    CookietopianMomentsOfMaybe,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::TimeMachine)))]
    SecondSeconds,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::TimeMachine)))]
    AdditionalClockHands,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::TimeMachine)))]
    Nostalgia,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::TimeMachine)))]
    SplitSeconds,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::TimeMachine)))]
    PatienceAbolished,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::TimeMachine)))]
    TimeproofUpholstery,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::TimeMachine)))]
    RectifyingAMistake,
    /* -------------------------------------------------------------------------- */
    /*                        Tiered: Antimatter Condenser                        */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::AntimatterCondenser)))]
    SugarBosons,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::AntimatterCondenser)))]
    StringTheory,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::AntimatterCondenser)))]
    LargeMacaronCollider,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::AntimatterCondenser)))]
    BigBangBake,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::AntimatterCondenser)))]
    ReverseCyclotrons,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::AntimatterCondenser)))]
    Nanocosmics,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::AntimatterCondenser)))]
    ThePulse,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::AntimatterCondenser)))]
    #[name(base = "Some Other Super-Tiny Fundamental Particle? Probably?")]
    SomeOtherSuperTinyFundamentalParticleProbably,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::AntimatterCondenser)))]
    QuantumComb,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::AntimatterCondenser)))]
    BakingNobelPrize,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::AntimatterCondenser)))]
    TheDefiniteMolecule,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::AntimatterCondenser)))]
    FlavorItself,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::AntimatterCondenser)))]
    DeliciousPull,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::AntimatterCondenser)))]
    EmployeeMinification,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::AntimatterCondenser)))]
    CandiedAtoms,
    /* -------------------------------------------------------------------------- */
    /*                                Tiered: Prism                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Prism)))]
    GemPolish,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Prism)))]
    #[name(base = "9th Color")]
    NinthColor,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Prism)))]
    ChocolateLight,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Prism)))]
    Grainbow,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Prism)))]
    PureCosmicLight,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Prism)))]
    #[name(base = "Glow-In-The-Dark")]
    GlowInTheDark,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Prism)))]
    LuxSanctorum,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Prism)))]
    ReverseShadows,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Prism)))]
    CrystalMirrors,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Prism)))]
    ReverseTheoryOfLight,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Prism)))]
    LightCaptureMeasures,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Prism)))]
    LightSpeedLimit,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Prism)))]
    #[name(base = "Occam's Laser")]
    OccamsLaser,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Prism)))]
    HyperblackPaint,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Prism)))]
    LabGogglesButLikeCoolShades,
    /* -------------------------------------------------------------------------- */
    /*                             Tiered: Chancemaker                            */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Chancemaker)))]
    YourLuckyCookie,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Chancemaker)))]
    #[name(base = "\"All Bets Are Off\" Magic Coin")]
    AllBetsAreOffMagicCoin,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Chancemaker)))]
    WinningLotteryTicket,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Chancemaker)))]
    #[name(base = "Four-Leaf Clover Field")]
    FourLeafCloverField,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Chancemaker)))]
    ARecipeBookAboutBooks,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Chancemaker)))]
    LeprechaunVillage,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Chancemaker)))]
    ImprobabilityDrive,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Chancemaker)))]
    Antisuperstistronics,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Chancemaker)))]
    Bunnypedes,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Chancemaker)))]
    RevisedProbabilistics,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Chancemaker)))]
    #[name(base = "0-Sided Dice")]
    ZeroSidedDice,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Chancemaker)))]
    ATouchOfDeterminism,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Chancemaker)))]
    OnAStreak,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Chancemaker)))]
    SilverLiningMaximization,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Chancemaker)))]
    #[name(base = "Gambler\'s Fallacy Fallacy")]
    GamblersFallacyFallacy,
    /* -------------------------------------------------------------------------- */
    /*                           Tiered: Fractal Engine                           */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::FractalEngine)))]
    Metabakeries,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::FractalEngine)))]
    MandelbrownSugar,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::FractalEngine)))]
    Fractoids,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::FractalEngine)))]
    NestedUniverseTheory,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::FractalEngine)))]
    MengerSpongeCake,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::FractalEngine)))]
    #[name(base = "One Particularly Good-Humored Cow")]
    OneParticularlyGoodHumoredCow,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::FractalEngine)))]
    ChocolateOuroboros,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::FractalEngine)))]
    Nested,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::FractalEngine)))]
    #[name(base = "Space-Filling Fibers")]
    SpaceFillingFibers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::FractalEngine)))]
    EndlessBookOfProse,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::FractalEngine)))]
    TheSetOfAllSets,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::FractalEngine)))]
    ThisUpgrade,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::FractalEngine)))]
    ABox,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::FractalEngine)))]
    MultiscaleProfiling,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::FractalEngine)))]
    TheMoreTheyStayTheSame,
    /* -------------------------------------------------------------------------- */
    /*                           Tiered: Rust Playground                          */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::RustPlayground)))]
    TheRustPlaygroundForDummies,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::RustPlayground)))]
    References,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::RustPlayground)))]
    BorrowChecker,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::RustPlayground)))]
    Turbofish,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::RustPlayground)))]
    SyntacticSugar,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::RustPlayground)))]
    BecomeCrab,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::RustPlayground)))]
    #[name(base = "Compile-Time Baking")]
    CompileTimeBaking,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::RustPlayground)))]
    #[name(base = "Cookies += 1")]
    CookiesPlusEquals1,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::RustPlayground)))]
    RustNightly,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::RustPlayground)))]
    InfiniteLoops,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::RustPlayground)))]
    UnsafePointers,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::RustPlayground)))]
    YourBiggestFans,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::RustPlayground)))]
    HackerShades,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::RustPlayground)))]
    UnsafeCointainmentVats,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::RustPlayground)))]
    CompilerIntrinsics,
    /* -------------------------------------------------------------------------- */
    /*                              Tiered: Idleverse                             */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::Idleverse)))]
    ManifestDestiny,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::Idleverse)))]
    TheMultiverseInANutshell,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::Idleverse)))]
    #[name(base = "All-Conversion")]
    AllConversion,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::Idleverse)))]
    MultiverseAgents,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::Idleverse)))]
    EscapePlan,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::Idleverse)))]
    GameDesign,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::Idleverse)))]
    SandboxUniverses,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::Idleverse)))]
    MultiverseWars,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::Idleverse)))]
    MobilePorts,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::Idleverse)))]
    EncapsulatedRealities,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::Idleverse)))]
    ExtrinsicClicking,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::Idleverse)))]
    UniversalIdling,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::Idleverse)))]
    BreakTheFifthWall,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::Idleverse)))]
    OppositeUniverse,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::Idleverse)))]
    TheOtherRoutesToRome,
    /* -------------------------------------------------------------------------- */
    /*                            Tiered: Cortex Baker                            */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::CortexBaker)))]
    PrincipledNeuralShackles,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::CortexBaker)))]
    Obey,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::CortexBaker)))]
    ASprinkleOfIrrationality,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::CortexBaker)))]
    FrontAndBackHemispheres,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::CortexBaker)))]
    NeuralNetworking,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::CortexBaker)))]
    CosmicBrainstorms,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::CortexBaker)))]
    Megatherapy,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::CortexBaker)))]
    SynapticLubricant,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::CortexBaker)))]
    Psychokinesis,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::CortexBaker)))]
    Spines,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::CortexBaker)))]
    Neuraforming,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::CortexBaker)))]
    EpistemologicalTrickery,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::CortexBaker)))]
    EveryPossibleIdea,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::CortexBaker)))]
    TheLandOfDreams,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::CortexBaker)))]
    IntellectualPropertyTheft,
    /* -------------------------------------------------------------------------- */
    /*                                 Tiered: You                                */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(0, Building::You)))]
    CloningVats,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(1, Building::You)))]
    EnergizedNutrients,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(2, Building::You)))]
    StuntDoubles,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(3, Building::You)))]
    CloneRecyclingPlant,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(4, Building::You)))]
    #[name(base = "Free-Range Clones")]
    FreeRangeClones,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(5, Building::You)))]
    GeneticTailoring,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(6, Building::You)))]
    PowerInDiversity,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(7, Building::You)))]
    #[name(base = "Self-Betterment")]
    SelfBetterment,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(8, Building::You)))]
    SourceControl,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(9, Building::You)))]
    UnitedWorkforce,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(10, Building::You)))]
    SafetyPatrols,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(11, Building::You)))]
    CloneRights,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(12, Building::You)))]
    OneBigFamily,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(13, Building::You)))]
    #[name(base = "Fine-Tuned Body Plans")]
    FineTunedBodyPlans,
    #[assoc(class = UpgradeClass::Tiered(Tiered::new(14, Building::You)))]
    ReadingYourClonesBedtimeStories,
    /* -------------------------------------------------------------------------- */
    /*                                   Grandma                                  */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Farm)))]
    FarmerGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Mine)))]
    MinerGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Factory)))]
    WorkerGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Bank)))]
    BankerGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Temple)))]
    PriestessGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::WizardTower)))]
    WitchGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Shipment)))]
    CosmicGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::AlchemyLab)))]
    TransmutedGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Portal)))]
    AlteredGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::TimeMachine)))]
    #[name(base = "Grandma's Grandmas")]
    GrandmasGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::AntimatterCondenser)))]
    Antigrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Prism)))]
    RainbowGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Chancemaker)))]
    LuckyGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::FractalEngine)))]
    Metagrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::RustPlayground)))]
    BinaryGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::Idleverse)))]
    AlternateGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::CortexBaker)))]
    BrainyGrandmas,
    #[assoc(class = UpgradeClass::GrandmaJob(GrandmaJob::new(Building::You)))]
    CloneGrandmas,
    /* -------------------------------------------------------------------------- */
    /*                                   Kitten                                   */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(13, 0.1, 9.0 * num::MILLION)))]
    KittenHelpers,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(25, 0.125, 9.0 * num::BILLION)))]
    KittenWorkers,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(50, 0.15, 90.0 * num::TRILLION)))]
    KittenEngineers,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(75, 0.175, 90.0 * num::QUADRILLION)))]
    KittenOverseers,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(100, 0.2, 900.0 * num::QUINTILLION)))]
    KittenManagers,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(125, 0.2, 900.0 * num::SEXTILLION)))]
    KittenAccountants,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(150, 0.2, 900.0 * num::SEPTILLION)))]
    KittenSpecialists,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(175, 0.2, 900.0 * num::OCTILLION)))]
    KittenExperts,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(200, 0.2, 900.0 * num::NONILLION)))]
    KittenConsulants,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(225, 0.175, 900.0 * num::DECILLION)))]
    KittenAssistantsToTheRegionalManager,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(250, 0.15, 900.0 * num::UNDECILLION)))]
    KittenMarketers,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(275, 0.125, 900.0 * num::DUODECILLION)))]
    KittenAnalysts,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(300, 0.115, 900.0 * num::TREDECILLION)))]
    KittenExecutives,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(325, 0.11, 900.0 * num::QUATTORDECILLION)))]
    KittenAdmins,
    #[assoc(class = UpgradeClass::Kitten(Kitten::new(350, 0.105, 900.0 * num::QUINDECILLION)))]
    KittenStrategists,
    /* -------------------------------------------------------------------------- */
    /*                          Research/Grandmapocalyse                          */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Research(Research::BingoCenterResearchFacility))]
    #[name(base = "Bingo Center/Research Facility")]
    BingoCenterResearchFacility,
    #[assoc(class = UpgradeClass::Research(Research::SpecializedChocolateChips))]
    SpecializedChocolateChips,
    #[assoc(class = UpgradeClass::Research(Research::DesignerCocoaBeans))]
    DesignerCocoaBeans,
    #[assoc(class = UpgradeClass::Research(Research::RitualRollingPins))]
    RitualRollingPins,
    #[assoc(class = UpgradeClass::Research(Research::UnderworldOvens))]
    UnderworldOvens,
    #[assoc(class = UpgradeClass::Research(Research::OneMind))]
    OneMind,
    #[assoc(class = UpgradeClass::Research(Research::ExoticNuts))]
    ExoticNuts,
    #[assoc(class = UpgradeClass::Research(Research::CommunalBrainsweep))]
    CommunalBrainsweep,
    #[assoc(class = UpgradeClass::Research(Research::ArcaneSugar))]
    ArcaneSugar,
    #[assoc(class = UpgradeClass::Research(Research::ElderPact))]
    ElderPact,
    #[assoc(class = UpgradeClass::Research(Research::SacrificialRollingPins))]
    SacrificialRollingPins,
    /* -------------------------------------------------------------------------- */
    /*                                  Switches                                  */
    /* -------------------------------------------------------------------------- */
    #[assoc(class = UpgradeClass::Switch(Switch::ElderPledge))]
    ElderPledge,
    #[assoc(class = UpgradeClass::Switch(Switch::ElderCovenant))]
    ElderCovenant,
    #[assoc(class = UpgradeClass::Switch(Switch::RevokeElderCovenant))]
    RevokeElderCovenant,
}

impl Upgrade {
    pub fn cost(&self) -> Cost {
        self.class().cost()
    }

    pub(crate) fn buy(&self, state: &mut State, changeset: &mut Changeset) {
        self.class().buy(state, changeset);
        changeset.available_upgrades = true;
    }

    pub fn effect_info(&self) -> UpgradeEffectInfo {
        self.class().effect_info()
    }

    pub fn should_add_to_owned(&self) -> bool {
        !matches!(self.class(), UpgradeClass::Switch(_))
    }
}

enum UpgradeClass {
    Tiered(Tiered),
    ClickAndCursor(ClickAndCursor),
    GrandmaJob(GrandmaJob),
    Kitten(Kitten),
    Research(Research),
    Switch(Switch),
}

impl UpgradeClass {
    fn cost(&self) -> Cost {
        match self {
            Self::Tiered(u) => u.cost(),
            Self::ClickAndCursor(u) => u.cost(),
            Self::GrandmaJob(u) => u.cost(),
            Self::Kitten(u) => u.cost(),
            Self::Research(u) => u.cost(),
            Self::Switch(u) => u.cost(),
        }
    }

    fn req(&self) -> Req {
        match self {
            Self::Tiered(u) => u.req(),
            Self::ClickAndCursor(u) => u.req(),
            Self::GrandmaJob(u) => u.req(),
            Self::Kitten(u) => u.req(),
            Self::Research(u) => u.req(),
            Self::Switch(u) => u.req(),
        }
    }

    fn buy(&self, state: &mut State, changeset: &mut Changeset) {
        match self {
            Self::Tiered(u) => u.buy(state, changeset),
            Self::ClickAndCursor(u) => u.buy(state, changeset),
            Self::GrandmaJob(u) => u.buy(state, changeset),
            Self::Kitten(u) => u.buy(state, changeset),
            Self::Research(u) => u.buy(state, changeset),
            Self::Switch(u) => u.buy(state, changeset),
        }
    }

    fn effect_info(&self) -> UpgradeEffectInfo {
        match self {
            Self::Tiered(u) => u.effect_info(),
            Self::ClickAndCursor(u) => u.effect_info(),
            Self::GrandmaJob(u) => u.effect_info(),
            Self::Kitten(u) => u.effect_info(),
            Self::Research(u) => u.effect_info(),
            Self::Switch(u) => u.effect_info(),
        }
    }
}
