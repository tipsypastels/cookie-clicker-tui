mod achievement;
mod building;
mod calc;
mod cookies;
mod cost;
mod grandmapocalypse;
mod milk;
mod req;
mod research;
mod sugar_lumps;
mod ticker;
mod upgrade;

pub use self::{
    achievement::{Achievement, AchievementReq},
    building::{Building, BuildingInfo},
    cost::Cost,
    grandmapocalypse::{Grandmapocalypse, GrandmapocalypsePhase},
    milk::{Milk, MilkFlavor},
    research::Research,
    sugar_lumps::SugarLumps,
    upgrade::{
        Upgrade, UpgradeEffectInfo, UpgradeInfoEffectResearch, UpgradeInfoEffectResearchWarning,
    },
};

use self::{
    achievement::Achievements,
    building::Buildings,
    cookies::Cookies,
    ticker::Ticker,
    upgrade::{AvailableUpgrades, OwnedUpgrades},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeSet, fmt};

pub struct Core {
    state: State,
    computed: Computed,
    everything_free: bool,
}

impl Core {
    pub fn new() -> Self {
        Self::from_state(State::new())
    }

    fn from_state(state: State) -> Self {
        let computed = Computed::new(&state);
        let everything_free = false;

        Self {
            state,
            computed,
            everything_free,
        }
    }

    pub fn cookies(&self) -> f64 {
        self.state.cookies.current()
    }

    pub fn cookies_all_time(&self) -> f64 {
        self.state.cookies.all_time()
    }

    pub fn cookies_all_time_from_clicking(&self) -> f64 {
        self.state.cookies.all_time_from_clicking()
    }

    pub fn cps(&self) -> f64 {
        self.computed.cps
    }

    pub fn milk(&self) -> &Milk {
        &self.state.milk
    }

    pub fn sugar_lumps(&self) -> &SugarLumps {
        &self.state.sugar_lumps
    }

    pub fn building_infos(&self) -> impl Iterator<Item = BuildingInfo> {
        self.state.buildings.infos()
    }

    pub fn building_info(&self, building: Building) -> BuildingInfo {
        self.state.buildings.info(building)
    }

    pub fn building_info_nth(&self, index: usize) -> BuildingInfo {
        self.state.buildings.info_nth(index)
    }

    pub fn owned_upgrades(&self) -> &BTreeSet<Upgrade> {
        self.state.owned_upgrades.as_set()
    }

    pub fn available_upgrades(&self) -> &[Upgrade] {
        &self.computed.available_upgrades
    }

    pub fn owned_achievements(&self) -> &BTreeSet<Achievement> {
        self.state.achievements.owned()
    }

    pub fn queued_achievement(&self) -> Option<Achievement> {
        self.state.achievements.queued()
    }

    pub fn research(&self) -> &Research {
        &self.state.research
    }

    pub fn grandmapocalypse(&self) -> &Grandmapocalypse {
        &self.state.grandmapocalypse
    }

    pub fn ticker(&self) -> Option<&'static str> {
        self.computed.ticker.text()
    }

    pub fn affordable(&self, cost: Cost) -> bool {
        self.everything_free || cost.affordable(&self.state)
    }

    pub fn click_cookie(&mut self) {
        self.state.cookies.gain_from_clicking(1.0);
    }

    pub fn give_building(&mut self, building: Building) {
        self.state.buildings.modify(building, |b| b.count += 1);
        self.computed.recalc_cps(&self.state);
        self.computed.recalc_available_upgrades(&self.state);
    }

    pub fn take_building(&mut self, building: Building) {
        self.state.buildings.modify(building, |b| b.count -= 1);
        self.computed.recalc_cps(&self.state);
        self.computed.recalc_available_upgrades(&self.state);
    }

    pub fn buy_building(&mut self, building: Building) -> bool {
        let cost = self.building_info(building).cost();

        if !self.affordable(cost) {
            return false;
        }

        if !self.everything_free {
            match cost {
                Cost::Cookies(cookies) => {
                    self.state.cookies.lose(cookies);
                }
            }
        }

        self.give_building(building);
        true
    }

    pub fn sell_building(&mut self, building: Building) -> bool {
        let info = self.building_info(building);

        if info.count() == 0 {
            return false;
        };

        if !self.everything_free {
            match info.sell_cost() {
                Cost::Cookies(cookies) => {
                    self.state.cookies.gain(cookies);
                }
            }
        }

        self.take_building(building);
        true
    }

    pub fn buy_upgrade(&mut self, upgrade: Upgrade) -> bool {
        if !self.computed.available_upgrades.contains(&upgrade) {
            return false;
        };

        if self.state.owned_upgrades.has(upgrade) {
            return false;
        }

        let cost = upgrade.cost();

        if !self.affordable(cost) {
            return false;
        }

        if !self.everything_free {
            match cost {
                Cost::Cookies(cookies) => {
                    self.state.cookies.lose(cookies);
                }
            }
        }

        self.state.owned_upgrades.add(upgrade);

        upgrade.buy(&mut self.state);

        self.computed.recalc_cps(&self.state);
        self.computed.recalc_available_upgrades(&self.state);

        true
    }

    pub fn make_everything_free(&mut self) {
        self.everything_free = true;
    }

    pub fn tick(&mut self) {
        self.state.tick(&self.computed);
        self.computed.tick(&self.state);
    }

    pub fn debug_cookies(&self) -> impl fmt::Debug {
        &self.state.cookies
    }

    pub fn debug_buildings(&self) -> impl fmt::Debug {
        &self.state.buildings
    }

    pub fn debug_buildings_flags(&self) -> impl fmt::Debug {
        self.state.buildings.flags()
    }

    pub fn debug_available_upgrades(&self) -> impl fmt::Debug {
        &self.computed.available_upgrades
    }

    pub fn debug_achievements(&self) -> impl fmt::Debug {
        &self.state.achievements
    }

    pub fn debug_ticker(&self) -> impl fmt::Debug {
        &self.computed.ticker
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for Core {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.state.serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Core {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        State::deserialize(de).map(Self::from_state)
    }
}

#[derive(Serialize, Deserialize)]
struct State {
    #[serde(default = "Cookies::new")]
    cookies: Cookies,
    #[serde(default = "Buildings::new")]
    buildings: Buildings,
    #[serde(default = "Milk::new")]
    milk: Milk,
    #[serde(default = "Achievements::new")]
    achievements: Achievements,
    #[serde(default = "OwnedUpgrades::new")]
    owned_upgrades: OwnedUpgrades,
    #[serde(default = "SugarLumps::new")]
    sugar_lumps: SugarLumps,
    #[serde(default = "Research::new")]
    research: Research,
    #[serde(default = "Grandmapocalypse::new")]
    grandmapocalypse: Grandmapocalypse,
}

impl State {
    fn new() -> Self {
        Self {
            cookies: Cookies::new(),
            buildings: Buildings::new(),
            milk: Milk::new(),
            achievements: Achievements::new(),
            owned_upgrades: OwnedUpgrades::new(),
            sugar_lumps: SugarLumps::new(),
            research: Research::new(),
            grandmapocalypse: Grandmapocalypse::new(),
        }
    }

    fn tick(&mut self, computed: &Computed) {
        self.cookies.tick(computed.cps);
        self.buildings.tick();
        self.milk.tick(self.achievements.owned().len() as _);
        self.research.tick();

        achievement::tick(self, computed);
        sugar_lumps::tick(self);
    }
}

struct Computed {
    cps: f64,
    ticker: Ticker,
    available_upgrades: AvailableUpgrades,
}

impl Computed {
    fn new(state: &State) -> Self {
        let cps = self::calc::cps(state);
        let ticker = Ticker::new(state);
        let available_upgrades = AvailableUpgrades::new(state);

        Self {
            cps,
            ticker,
            available_upgrades,
        }
    }

    fn tick(&mut self, state: &State) {
        self.ticker.tick(state);
        self.available_upgrades.tick(state);

        if state.research.just_completed() {
            self.recalc_available_upgrades(state);
        }
    }

    fn recalc_cps(&mut self, state: &State) {
        self.cps = self::calc::cps(state);
    }

    fn recalc_available_upgrades(&mut self, state: &State) {
        self.available_upgrades = AvailableUpgrades::new(state);
    }
}
