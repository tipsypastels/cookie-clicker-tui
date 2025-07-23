use crate::{Cost, calc};
use cookie_clicker_tui_utils::{enum_map, frames::FPS, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Predicates, Variants};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

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

pub struct Buildings {
    states: BuildingMap<BuildingState>,
    computeds: BuildingMap<BuildingComputed>,
}

impl Buildings {
    pub fn new() -> Self {
        Self::from_states(BuildingMap::default())
    }

    fn from_states(states: BuildingMap<BuildingState>) -> Self {
        let computeds = BuildingMap::new(|building| BuildingComputed::new(&states, building));

        Self { states, computeds }
    }

    pub fn tick(&mut self) {
        for building in Building::variants() {
            let cps = self.computeds.get(building).cps;
            let state = self.states.get_mut(building);
            state.cookies_all_time += cps / FPS;
        }
    }

    pub fn infos(&self) -> impl Iterator<Item = BuildingInfo> {
        Building::variants().map(|b| self.info(b))
    }

    pub fn info(&self, building: Building) -> BuildingInfo {
        BuildingInfo {
            building,
            state: self.states.get(building),
            computed: self.computeds.get(building),
        }
    }

    pub fn info_nth(&self, index: usize) -> BuildingInfo {
        self.info(Building::VARIANTS[index])
    }

    pub fn count(&self, building: Building) -> u16 {
        self.state(building).count
    }

    pub fn state(&self, building: Building) -> &BuildingState {
        self.states.get(building)
    }

    pub fn grandma_job_upgrade_count(&self) -> u16 {
        self.states.grandma_job_upgrade_count()
    }

    pub fn modify(&mut self, building: Building, f: impl FnOnce(&mut BuildingState)) {
        f(self.states.get_mut(building));
        self.recalc_computed(building);
    }

    pub fn recalc_computed(&mut self, building: Building) {
        *self.computeds.get_mut(building) = BuildingComputed::new(&self.states, building);
    }
}

impl fmt::Debug for Buildings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.infos()).finish()
    }
}

impl Serialize for Buildings {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.states.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Buildings {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        BuildingMap::deserialize(de).map(Self::from_states)
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
    pub addl_cps_per_owned_building: Vec<(Building, f64)>,
}

pub struct BuildingComputed {
    pub cost: f64,
    pub sell_cost: f64,
    pub cps: f64,
}

impl BuildingComputed {
    fn new(states: &BuildingMap<BuildingState>, building: Building) -> Self {
        let state = states.get(building);
        let cost = calc::building_cost(building, state.count);
        let sell_cost = calc::building_sell_cost(cost);

        let cps = calc::building_cps(calc::BuildingCps {
            building,
            building_class: match building {
                Building::Cursor => calc::BuildingCpsClass::Cursor,
                Building::Grandma => calc::BuildingCpsClass::Grandma {
                    grandma_job_upgrade_count: states.grandma_job_upgrade_count(),
                },
                _ => calc::BuildingCpsClass::Other {
                    grandma_count: if state.has_grandma_job_upgrade {
                        Some(states.grandma.count)
                    } else {
                        None
                    },
                },
            },
            count: state.count,
            tiered_upgrade_count: state.tiered_upgrade_count,
            addl_cps_per_owned_building_counts: state
                .addl_cps_per_owned_building
                .iter()
                .map(|(building, cps)| (states.get(*building).count, *cps)),
        });

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
    fn grandma_job_upgrade_count(&self) -> u16 {
        Building::variants()
            .map(|b| self.get(b).has_grandma_job_upgrade as u16)
            .sum()
    }
}
