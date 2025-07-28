use crate::{Cost, calc, macros};
use cookie_clicker_tui_utils::{enum_map, frames::FPS, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Predicates, Variants};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Buildings {
    state: BuildingsState,
    computed: BuildingsComputed,
}

impl Buildings {
    pub fn new() -> Self {
        Self::from_state(BuildingsState::default())
    }

    fn from_state(state: BuildingsState) -> Self {
        let computed = BuildingsComputed::new(&state);
        Self { state, computed }
    }

    pub fn tick(&mut self) {
        for building in Building::variants() {
            let cps = self.computed.buildings.get(building).cps;
            let state = self.state.buildings.get_mut(building);
            state.cookies_all_time += cps / FPS;
        }

        self.computed.total_count_just_changed = false;
    }

    pub fn infos(&self) -> impl Iterator<Item = BuildingInfo> {
        Building::variants().map(|b| self.info(b))
    }

    pub fn info(&self, building: Building) -> BuildingInfo {
        BuildingInfo {
            building,
            state: self.state.buildings.get(building),
            computed: self.computed.buildings.get(building),
        }
    }

    pub fn info_nth(&self, index: usize) -> BuildingInfo {
        self.info(Building::VARIANTS[index])
    }

    pub fn count(&self, building: Building) -> u16 {
        self.state(building).count
    }

    pub fn state(&self, building: Building) -> &BuildingState {
        self.state.buildings.get(building)
    }

    pub fn total_count(&self) -> u16 {
        self.computed.total_count
    }

    pub fn total_count_just_changed(&self) -> bool {
        self.computed.total_count_just_changed
    }

    pub fn debug_flags(&self) -> impl fmt::Debug {
        &self.state.flags
    }

    pub fn grandma_job_upgrade_count(&self) -> u16 {
        self.state.buildings.grandma_job_upgrade_count()
    }

    pub fn grandma_been_sold(&self) -> bool {
        self.state.flags.grandma_been_sold
    }

    pub fn modify_count(&mut self, building: Building, f: impl FnOnce(&mut u16)) {
        self.modify(building, |state| f(&mut state.count));
        self.computed.total_count = self.state.buildings.count();
        self.computed.total_count_just_changed = true;

        if self.state.flags.thousand_fingers_mult.is_some() && !building.is_cursor() {
            self.recompute(Building::Cursor);
        }

        if self.state.flags.grandma_has_elder_pact && building.is_portal() {
            self.recompute(Building::Grandma);
        }

        if building.is_grandma() {
            for building in Building::variants() {
                if self.state.buildings.get(building).has_grandma_job_upgrade {
                    self.recompute(building);
                }
            }
        }
    }

    pub fn modify_tiered_upgrade_count(&mut self, building: Building, f: impl FnOnce(&mut u16)) {
        self.modify(building, |state| f(&mut state.tiered_upgrade_count));
    }

    pub fn modify_has_grandma_job_upgrade(
        &mut self,
        building: Building,
        f: impl FnOnce(&mut bool),
    ) {
        self.modify(building, |state| f(&mut state.has_grandma_job_upgrade));
        self.recompute(Building::Grandma);
    }

    pub fn set_thousand_fingers_mult(&mut self, mult: Option<f64>) {
        self.state.flags.thousand_fingers_mult = mult;
        self.recompute(Building::Cursor);
    }

    pub fn set_grandma_has_bingo_center(&mut self, enable: bool) {
        self.state.flags.grandma_has_bingo_center = enable;
        self.recompute(Building::Grandma);
    }

    pub fn set_grandma_has_ritual_rolling_pins(&mut self, enable: bool) {
        self.state.flags.grandma_has_ritual_rolling_pins = enable;
        self.recompute(Building::Grandma);
    }

    pub fn set_grandma_has_one_mind(&mut self, enable: bool) {
        self.state.flags.grandma_has_one_mind = enable;
        self.recompute(Building::Grandma);
    }

    pub fn set_grandma_has_communal_brainsweep(&mut self, enable: bool) {
        self.state.flags.grandma_has_communal_brainsweep = enable;
        self.recompute(Building::Grandma);
    }

    pub fn set_grandma_has_elder_pact(&mut self, enable: bool) {
        self.state.flags.grandma_has_elder_pact = enable;
        self.recompute(Building::Grandma);
    }

    pub fn set_grandma_been_sold(&mut self, enable: bool) {
        self.state.flags.grandma_been_sold = enable;
    }

    fn modify(&mut self, building: Building, f: impl FnOnce(&mut BuildingState)) {
        f(self.state.buildings.get_mut(building));
        self.recompute(building);
    }

    fn recompute(&mut self, building: Building) {
        *self.computed.buildings.get_mut(building) =
            BuildingComputed::new(&self.state.buildings, &self.state.flags, building);
    }
}

impl fmt::Debug for Buildings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.infos()).finish()
    }
}

macros::serialize_via_state!(Buildings => BuildingsState as |b| b.state);
macros::deserialize_via_state!(Buildings => BuildingsState as Buildings::from_state);

#[derive(Serialize, Deserialize, Default)]
struct BuildingsState {
    #[serde(flatten)]
    buildings: BuildingMap<BuildingState>,
    flags: BuildingsFlags,
}

struct BuildingsComputed {
    buildings: BuildingMap<BuildingComputed>,
    total_count: u16,
    total_count_just_changed: bool,
}

impl BuildingsComputed {
    fn new(state: &BuildingsState) -> Self {
        Self {
            buildings: BuildingMap::new(|b| {
                BuildingComputed::new(&state.buildings, &state.flags, b)
            }),
            total_count: state.buildings.count(),
            total_count_just_changed: true,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct BuildingsFlags {
    thousand_fingers_mult: Option<f64>,
    grandma_has_bingo_center: bool,
    grandma_has_ritual_rolling_pins: bool,
    grandma_has_one_mind: bool,
    grandma_has_communal_brainsweep: bool,
    grandma_has_elder_pact: bool,
    grandma_been_sold: bool,
}

#[derive(
    Assoc,
    Name,
    Variants,
    Predicates,
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
#[func(pub(crate) const fn base_cost(self) -> f64)]
#[func(pub(crate) const fn base_cps(self) -> f64)]
#[name(base = "title case")]
#[name(extra(plural = "title case plural"))]
#[name(extra(lower = "title case lower"))]
#[name(extra(lower_plural = "title case lower plural"))]
#[name(pluralizer(base, plural))]
#[name(pluralizer(lower, lower_plural))]
pub enum Building {
    #[assoc(base_cost = 15.0, base_cps = 0.1)]
    Cursor,
    #[assoc(base_cost = 100.0, base_cps = 1.0)]
    Grandma,
    #[assoc(base_cost = 1100.0, base_cps = 8.0)]
    Farm,
    #[assoc(base_cost = 12.0 * num::THOUSAND, base_cps = 47.0)]
    Mine,
    #[assoc(base_cost = 130.0 * num::THOUSAND, base_cps = 260.0)]
    #[name(plural = "Factories", lower_plural = "factories")]
    Factory,
    #[assoc(base_cost = 1.4 * num::MILLION, base_cps = 1_400.0)]
    Bank,
    #[assoc(base_cost = 20.0 * num::MILLION, base_cps = 7_800.0)]
    Temple,
    #[assoc(base_cost = 330.0 * num::MILLION, base_cps = 44.0 * num::THOUSAND)]
    WizardTower,
    #[assoc(base_cost = 5.1 * num::BILLION, base_cps = 260.0 * num::THOUSAND)]
    Shipment,
    #[assoc(base_cost = 75.0 * num::BILLION, base_cps = 1.6 * num::MILLION)]
    AlchemyLab,
    #[assoc(base_cost = 1.0 * num::TRILLION, base_cps = 10.0 * num::MILLION)]
    Portal,
    #[assoc(base_cost = 14.0 * num::TRILLION, base_cps = 64.0 * num::MILLION)]
    TimeMachine,
    #[assoc(base_cost = 170.0 * num::TRILLION, base_cps = 430.0 * num::MILLION)]
    AntimatterCondenser,
    #[assoc(base_cost = 2.1 * num::QUADRILLION, base_cps = 2.9 * num::BILLION)]
    Prism,
    #[assoc(base_cost = 26.0 * num::QUADRILLION, base_cps = 21.0 * num::BILLION)]
    Chancemaker,
    #[assoc(base_cost = 310.0 * num::QUADRILLION, base_cps = 150.0 * num::BILLION)]
    FractalEngine,
    #[assoc(base_cost = 71.0 * num::QUINTILLION, base_cps = 1.1 * num::TRILLION)]
    RustPlayground,
    #[assoc(base_cost = 12.0 * num::SEXTILLION, base_cps = 8.3 * num::TRILLION)]
    Idleverse,
    #[assoc(base_cost = 1.9 * num::SEPTILLION, base_cps = 64.0 * num::TRILLION)]
    CortexBaker,
    #[assoc(base_cost = 540.0 * num::SEPTILLION, base_cps = 510.0 * num::TRILLION)]
    #[name(plural = "of You", lower_plural = "of you")]
    You,
}

impl Building {
    pub fn nth(index: usize) -> Option<Self> {
        Self::VARIANTS.get(index).copied()
    }
}

pub struct BuildingInfo<'a> {
    building: Building,
    state: &'a BuildingState,
    computed: &'a BuildingComputed,
}

impl BuildingInfo<'_> {
    pub fn building(&self) -> Building {
        self.building
    }

    pub fn count(&self) -> u16 {
        self.state.count
    }

    pub fn cookies_all_time(&self) -> f64 {
        self.state.cookies_all_time
    }

    pub fn tiered_upgrade_count(&self) -> u16 {
        self.state.tiered_upgrade_count
    }

    pub fn has_grandma_job_upgrade(&self) -> bool {
        self.state.has_grandma_job_upgrade
    }

    pub fn cost(&self) -> Cost {
        Cost::Cookies(self.computed.cost)
    }

    pub fn sell_cost(&self) -> Cost {
        Cost::Cookies(self.computed.sell_cost)
    }

    pub fn cps(&self) -> f64 {
        self.computed.cps
    }
}

impl fmt::Debug for BuildingInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.building.name())
            .field("count", &self.state.count)
            .field("cps", &self.computed.cps)
            .field("cost", &self.computed.cost)
            .field("sell_cost", &self.computed.sell_cost)
            .field("cookies_all_time", &self.state.cookies_all_time)
            .field("tiered_upgrade_count", &self.state.tiered_upgrade_count)
            .field(
                "has_grandma_job_upgrade",
                &self.state.has_grandma_job_upgrade,
            )
            .finish()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct BuildingState {
    pub count: u16,
    pub cookies_all_time: f64,
    pub tiered_upgrade_count: u16,
    pub has_grandma_job_upgrade: bool,
}

struct BuildingComputed {
    cost: f64,
    sell_cost: f64,
    cps: f64,
}

impl BuildingComputed {
    fn new(
        buildings: &BuildingMap<BuildingState>,
        flags: &BuildingsFlags,
        building: Building,
    ) -> Self {
        let state = buildings.get(building);
        let cost = calc::building_cost(building, state.count);
        let sell_cost = calc::building_sell_cost(cost);

        let building_class = match building {
            Building::Cursor => calc::BuildingCpsClass::Cursor {
                thousand_fingers: flags
                    .thousand_fingers_mult
                    .map(|mult| calc::ThousandFingers {
                        non_cursor_buildings_count: buildings.count() - buildings.cursor.count,
                        mult,
                    }),
            },
            Building::Grandma => calc::BuildingCpsClass::Grandma {
                has_bingo_center: flags.grandma_has_bingo_center,
                has_ritual_rolling_pins: flags.grandma_has_ritual_rolling_pins,
                has_one_mind: flags.grandma_has_one_mind,
                has_communal_brainsweep: flags.grandma_has_communal_brainsweep,
                elder_pact_portal_count: flags
                    .grandma_has_elder_pact
                    .then_some(buildings.portal.count),
                job_upgrade_count: buildings.grandma_job_upgrade_count(),
            },
            _ => calc::BuildingCpsClass::Other {
                grandma_count: state
                    .has_grandma_job_upgrade
                    .then_some(buildings.grandma.count),
            },
        };

        let cps = calc::building_cps(
            building,
            building_class,
            state.count,
            state.tiered_upgrade_count,
        );

        Self {
            cost,
            sell_cost,
            cps,
        }
    }
}

enum_map! {
    #[derive(Serialize, Deserialize, Default, Debug)]
    struct BuildingMap of Building {
        cursor: Cursor,
        grandma: Grandma,
        farm: Farm,
        mine: Mine,
        factory: Factory,
        bank: Bank,
        temple: Temple,
        wizard_tower: WizardTower,
        shipment: Shipment,
        alchemy_lab: AlchemyLab,
        portal: Portal,
        time_machine: TimeMachine,
        antimatter_condenser: AntimatterCondenser,
        prism: Prism,
        chancemaker: Chancemaker,
        fractal_engine: FractalEngine,
        rust_playground: RustPlayground,
        idleverse: Idleverse,
        cortex_baker: CortexBaker,
        you: You,
    }
}

impl BuildingMap<BuildingState> {
    fn count(&self) -> u16 {
        Building::variants().map(|b| self.get(b).count).sum()
    }

    fn grandma_job_upgrade_count(&self) -> u16 {
        Building::variants()
            .map(|b| self.get(b).has_grandma_job_upgrade as u16)
            .sum()
    }
}
