use crate::num;
use enum_assoc::Assoc;
use enum_fun::{Name, Variants};

#[derive(Debug)]
pub struct Building {
    kind: BuildingKind,
    count: u16,
}

#[derive(Assoc, Name, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub(crate) const fn base_cost(self) -> f64)]
#[func(pub(crate) const fn base_cps(self) -> f64)]
pub enum BuildingKind {
    #[assoc(base_cost = 15., base_cps = 0.1)]
    Cursor,
    #[assoc(base_cost = 100., base_cps = 1.)]
    Grandma,
    #[assoc(base_cost = 1100., base_cps = 8.)]
    Farm,
    #[assoc(base_cost = 1_100., base_cps = 47.)]
    Mine,
    #[assoc(base_cost = 130. * num::THOUSAND, base_cps = 260.)]
    Factory,
    #[assoc(base_cost = 1.4 * num::MILLION, base_cps = 1_400.)]
    Bank,
    #[assoc(base_cost = 20. * num::MILLION, base_cps = 7_800.)]
    Temple,
    #[assoc(base_cost = 330. * num::MILLION, base_cps = 44. * num::THOUSAND)]
    WizardTower,
    #[assoc(base_cost = 5.1 * num::BILLION, base_cps = 260. * num::THOUSAND)]
    Shipment,
    #[assoc(base_cost = 75. * num::BILLION, base_cps = 1.6 * num::MILLION)]
    AlchemyLab,
    #[assoc(base_cost = 1. * num::TRILLION, base_cps = 10. * num::MILLION)]
    Portal,
    #[assoc(base_cost = 14. * num::TRILLION, base_cps = 64. * num::MILLION)]
    TimeMachine,
    #[assoc(base_cost = 170. * num::TRILLION, base_cps = 430. * num::MILLION)]
    AntimatterCondenser,
    #[assoc(base_cost = 2.1 * num::QUADRILLION, base_cps = 2.9 * num::BILLION)]
    Prism,
    #[assoc(base_cost = 26. * num::QUADRILLION, base_cps = 21. * num::BILLION)]
    Chancemaker,
    #[assoc(base_cost = 310. * num::QUADRILLION, base_cps = 150. * num::BILLION)]
    FractalEngine,
    #[assoc(base_cost = 71. * num::QUINTILLION, base_cps = 1.1 * num::TRILLION)]
    RustPlayground,
    #[assoc(base_cost = 12. * num::SEXTILLION, base_cps = 8.3 * num::TRILLION)]
    Idleverse,
    #[assoc(base_cost = 1.9 * num::SEPTILLION, base_cps = 64. * num::TRILLION)]
    CortexBaker,
    #[assoc(base_cost = 0. * num::SEPTILLION, base_cps = 510. * num::TRILLION)]
    #[name(plural = "of You")]
    You,
}
