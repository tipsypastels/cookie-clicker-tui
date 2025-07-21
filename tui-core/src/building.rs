use crate::calc;
use cookie_clicker_tui_utils::{frames::FPS, num};
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};
use std::collections::HashMap;

#[derive(Assoc, Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug)]
pub struct Buildings {
    states: HashMap<Building, BuildingState>,
    computeds: HashMap<Building, BuildingComputed>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            computeds: HashMap::new(),
        }
    }

    pub fn infos(&self) -> impl Iterator<Item = BuildingInfo> {
        Building::variants().map(|b| self.info(b))
    }

    pub fn info(&self, building: Building) -> BuildingInfo {
        BuildingInfo {
            building,
            state: self.state(building),
            computed: self.computed(building),
        }
    }

    pub fn info_nth(&self, index: usize) -> BuildingInfo {
        self.info(Building::VARIANTS[index])
    }

    pub fn modify(&mut self, building: Building, f: impl Fn(&mut BuildingState) + Clone) {
        let state = *self
            .states
            .entry(building)
            .and_modify(f.clone())
            .or_insert_with(|| {
                let mut state = BuildingState::default();
                f(&mut state);
                state
            });

        let computed = self.compute(building, state);
        self.computeds.insert(building, computed);
    }

    pub fn tick(&mut self) {
        for building in Building::variants() {
            let Some(cps) = self.computeds.get(&building).map(|c| c.cps) else {
                continue;
            };
            let Some(state) = self.states.get_mut(&building) else {
                continue;
            };
            state.cookies_all_time += cps / FPS;
        }
    }

    pub fn state(&self, building: Building) -> BuildingState {
        self.states.get(&building).copied().unwrap_or_default()
    }

    pub fn computed(&self, building: Building) -> BuildingComputed {
        self.computeds
            .get(&building)
            .copied()
            .unwrap_or_else(|| self.compute(building, self.state(building)))
    }

    fn compute(&self, building: Building, state: BuildingState) -> BuildingComputed {
        let cost = calc::building_cost(building, state.count);
        let cps = calc::building_cps(calc::BuildingCps {
            building,
            building_class: match building {
                Building::Cursor => calc::BuildingCpsClass::Cursor,
                Building::Grandma => calc::BuildingCpsClass::Grandma {
                    grandma_co_tiered_upgrade_count: self.grandma_co_tiered_upgrade_count(),
                },
                _ => calc::BuildingCpsClass::Other {
                    grandma_count_for_co_tiered_upgrade: if state.has_grandma_co_tiered_upgrade {
                        Some(self.state(Building::Grandma).count)
                    } else {
                        None
                    },
                },
            },
            count: state.count,
            simple_tiered_upgrade_count: state.simple_tiered_upgrade_count,
        });

        BuildingComputed { cost, cps }
    }

    fn grandma_co_tiered_upgrade_count(&self) -> u16 {
        self.states
            .values()
            .map(|s| s.has_grandma_co_tiered_upgrade as u16)
            .sum()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BuildingInfo {
    building: Building,
    state: BuildingState,
    computed: BuildingComputed,
}

impl BuildingInfo {
    pub fn building(&self) -> Building {
        self.building
    }

    pub fn count(&self) -> u16 {
        self.state.count
    }

    pub fn cookies_all_time(&self) -> f64 {
        self.state.cookies_all_time
    }

    pub fn simple_tiered_upgrade_count(&self) -> u16 {
        self.state.simple_tiered_upgrade_count
    }

    pub fn has_grandma_co_tiered_upgrade(&self) -> bool {
        self.state.has_grandma_co_tiered_upgrade
    }

    pub fn cost(&self) -> f64 {
        self.computed.cost
    }

    pub fn cps(&self) -> f64 {
        self.computed.cps
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct BuildingState {
    pub count: u16,
    pub cookies_all_time: f64,
    pub simple_tiered_upgrade_count: u16,
    pub has_grandma_co_tiered_upgrade: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct BuildingComputed {
    pub cost: f64,
    pub cps: f64,
}

macro_rules! all_the_buildings {
    ($macro:ident) => {
        $macro!(
            Cursor,
            Grandma,
            Farm,
            Mine,
            Factory,
            Bank,
            Temple,
            WizardTower,
            Shipment,
            AlchemyLab,
            Portal,
            TimeMachine,
            AntimatterCondenser,
            Prism,
            Chancemaker,
            FractalEngine,
            RustPlayground,
            Idleverse,
            CortexBaker,
            You,
        )
    };
}

pub(crate) use all_the_buildings;
