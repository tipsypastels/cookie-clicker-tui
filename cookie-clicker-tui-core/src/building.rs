use crate::num;
use cookie_clicker_tui_macros::{Name, Variants};
use enum_assoc::Assoc;

#[derive(Debug)]
pub struct Building {
    pub kind: BuildingKind,
    pub count: u16,
}

#[derive(Assoc, Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub(crate) const fn base_cost(self) -> f64)]
#[func(pub(crate) const fn base_cps(self) -> f64)]
pub enum BuildingKind {
    #[assoc(base_cost = 15.0, base_cps = 0.1)]
    Cursor,
    #[assoc(base_cost = 100.0, base_cps = 1.0)]
    Grandma,
    #[assoc(base_cost = 1000.0, base_cps = 10.0)]
    Farm,
    #[assoc(base_cost = 1_100.0, base_cps = 47.0)]
    Mine,
    #[assoc(base_cost = 130.0 * num::THOUSAND, base_cps = 260.0)]
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
    #[assoc(base_cost = 0.0 * num::SEPTILLION, base_cps = 510.0 * num::TRILLION)]
    #[name(plural = "of You")]
    You,
}
