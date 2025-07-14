use crate::num::big;
use enum_assoc::Assoc;
use std::collections::HashMap;

macro_rules! names {
    ($s:literal) => {
        ($s, concat!($s, "s"))
    };
}

#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(const fn names(self) -> (&'static str, &'static str))]
#[func(pub const fn base_cost(self) -> f64)]
#[func(pub const fn base_cps(self) -> f64)]
pub enum Building {
    #[assoc(names = names!("Cursor"), base_cost = 15.0, base_cps = 0.1)]
    Cursor,
    #[assoc(names = names!("Grandma"), base_cost = 100.0, base_cps = 1.0)]
    Grandma,
    #[assoc(names = names!("Farm"), base_cost = 1000.0, base_cps = 10.0)]
    Farm,
    #[assoc(names = names!("Mine"), base_cost = 1_100.0, base_cps = 47.0)]
    Mine,
    #[assoc(names = names!("Factory"), base_cost = big!(130.0 thousand f), base_cps = 260.0)]
    Factory,
    #[assoc(names = names!("Bank"), base_cost = big!(1.4 million f), base_cps = 1_400.0)]
    Bank,
    #[assoc(names = names!("Temple"), base_cost = big!(20.0 million f), base_cps = 7_800.0)]
    Temple,
    #[assoc(names = names!("Wizard Tower"), base_cost = big!(330.0 million f), base_cps = big!(44.0 thousand f))]
    WizardTower,
    #[assoc(names = names!("Shipment"), base_cost = big!(5.1 billion f), base_cps = big!(260.0 thousand f))]
    Shipment,
    #[assoc(names = names!("Alchemy Lab"), base_cost = big!(75.0 billion f), base_cps = big!(1.6 million f))]
    AlchemyLab,
    #[assoc(names = names!("Portal"), base_cost = big!(1.0 trillion f), base_cps = big!(10.0 million f))]
    Portal,
    #[assoc(names = names!("Time Machine"), base_cost = big!(14.0 trillion f), base_cps = big!(64.0 million f))]
    TimeMachine,
    #[assoc(names = names!("Antimatter Condenser"), base_cost = big!(170.0 trillion f), base_cps = big!(430.0 million f))]
    AntimatterCondenser,
    #[assoc(names = names!("Prism"), base_cost = big!(2.1 quadrillion f), base_cps = big!(2.9 billion f))]
    Prism,
    #[assoc(names = names!("Chancemaker"), base_cost = big!(26.0 quadrillion f), base_cps = big!(21.0 billion f))]
    Chancemaker,
    #[assoc(names = names!("Fractal Engine"), base_cost = big!(310.0 quadrillion f), base_cps = big!(150.0 billion f))]
    FractalEngine,
    #[assoc(names = names!("Rust Playground"), base_cost = big!(71.0 quintillion f), base_cps = big!(1.1 trillion f))]
    RustPlayground,
    #[assoc(names = names!("Idleverse"), base_cost = big!(12.0 sextillion f), base_cps = big!(8.3 trillion f))]
    Idleverse,
    #[assoc(names = names!("Cortex Baker"), base_cost = big!(1.9 septillion f), base_cps = big!(64.0 trillion f))]
    CortexBaker,
    #[assoc(names = ("You", "of You"), base_cost = big!(540.0 septillion f), base_cps = big!(510.0 trillion f))]
    You,
}

impl Building {
    pub const ALL: [Self; 20] = [
        Self::Cursor,
        Self::Grandma,
        Self::Farm,
        Self::Mine,
        Self::Factory,
        Self::Bank,
        Self::Temple,
        Self::WizardTower,
        Self::Shipment,
        Self::AlchemyLab,
        Self::Portal,
        Self::TimeMachine,
        Self::AntimatterCondenser,
        Self::Prism,
        Self::Chancemaker,
        Self::FractalEngine,
        Self::RustPlayground,
        Self::Idleverse,
        Self::CortexBaker,
        Self::You,
    ];

    pub fn get(i: usize) -> Option<Self> {
        Self::ALL.get(i).copied()
    }

    pub const fn name(self) -> &'static str {
        self.names().0
    }

    pub const fn name_plural(self) -> &'static str {
        self.names().1
    }

    pub const fn name_pluralized(self, count: usize) -> &'static str {
        if count == 1 {
            self.name()
        } else {
            self.name_plural()
        }
    }
}

#[derive(Debug)]
pub struct Buildings {
    counts: HashMap<Building, u16>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn buy(&mut self, building: Building) {
        self.counts
            .entry(building)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn cps(&self) -> f64 {
        self.iter()
            .map(|(building, count)| building.base_cps() * count as f64)
            .sum()
    }

    pub fn count(&self, building: Building) -> u16 {
        self.counts.get(&building).copied().unwrap_or_default()
    }

    pub fn cost(&self, building: Building) -> f64 {
        building.base_cost() * 1.15f64.powi(self.count(building).into())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Building, u16)> {
        Building::ALL.iter().map(|&b| (b, self.count(b)))
    }
}
