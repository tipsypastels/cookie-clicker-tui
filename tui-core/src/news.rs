use crate::{
    State,
    building::Building,
    grandmapocalypse::GrandmapocalypsePhase,
    req::{Cmp, Req},
};
use cookie_clicker_tui_utils::num;
use enum_assoc::Assoc;
use enum_fun::Variants;
use rand::seq::IndexedRandom;

#[allow(non_camel_case_types)]
#[derive(Assoc, Variants, Debug, Copy, Clone)]
#[func(const fn req(&self) -> Req)]
pub enum NewsEntry {
    /* -------------------------------------------------------------------------- */
    /*                                Cookie Based                                */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = Req::CookiesAllTime(Cmp::Below(5.0)))]
    CookiesAllTime_Below_5,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(5.0, 50.0)))]
    CookiesAllTime_5_To_50,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(50.0, 100.0)))]
    CookiesAllTime_50_To_100,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(100.0, 500.0)))]
    CookiesAllTime_100_To_500,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(500.0, 1.0 * num::THOUSAND)))]
    CookiesAllTime_500_To_1K,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(1.0 * num::THOUSAND, 5.0 * num::THOUSAND)))]
    CookiesAllTime_1K_To_5K,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(5.0 * num::THOUSAND, 10.0 * num::THOUSAND)))]
    CookiesAllTime_5K_To_10K,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(10.0 * num::THOUSAND, 50.0 * num::THOUSAND)))]
    CookiesAllTime_10K_To_50K,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(50.0 * num::THOUSAND, 100.0 * num::THOUSAND)))]
    CookiesAllTime_50K_To_100K,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(100.0 * num::THOUSAND, 500.0 * num::THOUSAND)))]
    CookiesAllTime_100K_To_500K,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(500.0 * num::THOUSAND, 1.0 * num::MILLION)))]
    CookiesAllTime_500K_To_1M,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(1.0 * num::MILLION, 5.0 * num::MILLION)))]
    CookiesAllTime_1M_To_5M,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(5.0 * num::MILLION, 10.0 * num::MILLION)))]
    CookiesAllTime_5M_To_10M,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(10.0 * num::MILLION, 50.0 * num::MILLION)))]
    CookiesAllTime_10M_To_50M,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(50.0 * num::MILLION, 100.0 * num::MILLION)))]
    CookiesAllTime_50M_To_100M,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(100.0 * num::MILLION, 500.0 * num::MILLION)))]
    CookiesAllTime_100M_To_500M,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(500.0 * num::MILLION, 1.0 * num::BILLION)))]
    CookiesAllTime_500M_To_1B,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(1.0 * num::BILLION, 5.0 * num::BILLION)))]
    CookiesAllTime_1B_To_5B,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(5.0 * num::BILLION, 10.0 * num::BILLION)))]
    CookiesAllTime_5B_To_10B,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(10.0 * num::BILLION, 50.0 * num::BILLION)))]
    CookiesAllTime_10B_To_50B,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(50.0 * num::BILLION, 100.0 * num::BILLION)))]
    CookiesAllTime_50B_To_100B,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(100.0 * num::BILLION, 500.0 * num::BILLION)))]
    CookiesAllTime_100B_To_500B,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(500.0 * num::BILLION, 1.0 * num::TRILLION)))]
    CookiesAllTime_500B_To_1T,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(1.0 * num::TRILLION, 5.0 * num::TRILLION)))]
    CookiesAllTime_1T_To_5T,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(5.0 * num::TRILLION, 10.0 * num::TRILLION)))]
    CookiesAllTime_5T_To_10T,
    #[assoc(req = Req::CookiesAllTime(Cmp::Range(10.0 * num::TRILLION, 100.0 * num::TRILLION)))]
    CookiesAllTime_10T_To_100T,
    #[assoc(req = Req::CookiesAllTime(Cmp::Above(100.0 * num::TRILLION)))]
    CookiesAllTime_Above_100T,
    /* -------------------------------------------------------------------------- */
    /*                               Building Based                               */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = Req::BuildingCount(Building::Grandma, Cmp::AboveOrEq(1)))]
    Building_Grandma_1,
    #[assoc(req = Req::BuildingCount(Building::Grandma, Cmp::AboveOrEq(50)))]
    Building_Grandma_50,
    #[assoc(req = Req::BuildingCount(Building::Farm, Cmp::AboveOrEq(1)))]
    Building_Farm_1,
    #[assoc(req = Req::BuildingCount(Building::Mine, Cmp::AboveOrEq(1)))]
    Building_Mine_1,
    #[assoc(req = Req::BuildingCount(Building::Factory, Cmp::AboveOrEq(1)))]
    Building_Factory_1,
    #[assoc(req = Req::BuildingCount(Building::Bank, Cmp::AboveOrEq(1)))]
    Building_Bank_1,
    #[assoc(req = Req::BuildingCount(Building::Temple, Cmp::AboveOrEq(1)))]
    Building_Temple_1,
    #[assoc(req = Req::BuildingCount(Building::WizardTower, Cmp::AboveOrEq(1)))]
    Building_WizardTower_1,
    #[assoc(req = Req::BuildingCount(Building::Shipment, Cmp::AboveOrEq(1)))]
    Building_Shipment_1,
    #[assoc(req = Req::BuildingCount(Building::AlchemyLab, Cmp::AboveOrEq(1)))]
    Building_AlchemyLab_1,
    #[assoc(req = Req::BuildingCount(Building::Portal, Cmp::AboveOrEq(1)))]
    Building_Portal_1,
    #[assoc(req = Req::BuildingCount(Building::TimeMachine, Cmp::AboveOrEq(1)))]
    Building_TimeMachine_1,
    #[assoc(req = Req::BuildingCount(Building::AntimatterCondenser, Cmp::AboveOrEq(1)))]
    Building_AntimatterCondenser_1,
    #[assoc(req = Req::BuildingCount(Building::Prism, Cmp::AboveOrEq(1)))]
    Building_Prism_1,
    #[assoc(req = Req::BuildingCount(Building::Chancemaker, Cmp::AboveOrEq(1)))]
    Building_Chancemaker_1,
    #[assoc(req = Req::BuildingCount(Building::FractalEngine, Cmp::AboveOrEq(1)))]
    Building_FractalEngine_1,
    #[assoc(req = Req::BuildingCount(Building::RustPlayground, Cmp::AboveOrEq(1)))]
    Building_RustPlayground_1,
    #[assoc(req = Req::BuildingCount(Building::Idleverse, Cmp::AboveOrEq(1)))]
    Building_Idleverse_1,
    #[assoc(req = Req::BuildingCount(Building::CortexBaker, Cmp::AboveOrEq(1)))]
    Building_CortexBaker_1,
    #[assoc(req = Req::BuildingCount(Building::You, Cmp::AboveOrEq(1)))]
    Building_You_1,
    /* -------------------------------------------------------------------------- */
    /*                           Grandmapocalypse Based                           */
    /* -------------------------------------------------------------------------- */
    #[assoc(req = Req::GrandmapocalypsePhase(GrandmapocalypsePhase::Awoken))]
    Grandmapocalypse_Awoken,
    #[assoc(req = Req::GrandmapocalypsePhase(GrandmapocalypsePhase::Displeased))]
    Grandmapocalypse_Displeased,
    #[assoc(req = Req::GrandmapocalypsePhase(GrandmapocalypsePhase::Angered))]
    Grandmapocalypse_Angered,
    #[assoc(req = Req::GrandmapocalypseAppeased())]
    Grandmapocalypse_Appeased,
}

pub fn get_entry(state: &State) -> Option<NewsEntry> {
    NewsEntry::variants()
        .filter(|e| e.req().check(state))
        .collect::<Vec<_>>()
        .choose(&mut rand::rng())
        .copied()
}
