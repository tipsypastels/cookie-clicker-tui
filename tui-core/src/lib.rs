mod achievement;
mod building;
mod calc;
mod cookies;
mod milk;
mod req;
mod ticker;
mod upgrade;

pub use self::{
    achievement::{Achievement, AchievementReq},
    building::{Building, BuildingInfo},
    milk::{Milk, MilkFlavor},
    upgrade::{Upgrade, UpgradeEffectInfo},
};

use self::{
    achievement::Achievements, building::Buildings, cookies::Cookies, ticker::Ticker,
    upgrade::Upgrades,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Core {
    state: State,
    computed: Computed,
}

impl Core {
    pub fn new() -> Self {
        Self::from_state(State::new())
    }

    fn from_state(state: State) -> Self {
        let computed = Computed::new(&state);

        Self { state, computed }
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

    pub fn building_infos(&self) -> impl Iterator<Item = BuildingInfo> {
        self.state.buildings.infos()
    }

    pub fn building_info(&self, building: Building) -> BuildingInfo {
        self.state.buildings.info(building)
    }

    pub fn building_info_nth(&self, index: usize) -> BuildingInfo {
        self.state.buildings.info_nth(index)
    }

    pub fn upgrades(&self) -> &[Upgrade] {
        &self.computed.upgrades
    }

    pub fn owned_achievements(&self) -> &BTreeSet<Achievement> {
        self.state.achievements.owned()
    }

    pub fn queued_achievement(&self) -> Option<Achievement> {
        self.state.achievements.queued()
    }

    pub fn ticker(&self) -> Option<&'static str> {
        self.computed.ticker.text()
    }

    pub fn click_cookie(&mut self) {
        self.state.cookies.gain_from_clicking(1.0);
    }

    pub fn give_free_building(&mut self, building: Building) {
        self.state.buildings.modify(building, |b| b.count += 1);
        self.computed.recalc_cps(&self.state);
        self.computed.recalc_upgrades(&self.state);
    }

    pub fn buy_building(&mut self, building: Building) -> bool {
        let cost = self.building_info(building).cost();

        if cost > self.state.cookies.current() {
            return false;
        }

        self.state.cookies.lose(cost);
        self.give_free_building(building);

        true
    }

    pub fn buy_upgrade(&mut self, index: usize) -> bool {
        let Some(upgrade) = self.computed.upgrades.get(index) else {
            return false;
        };

        let cost = upgrade.cost();

        if cost > self.state.cookies.current() {
            return false;
        }

        self.state.cookies.lose(cost);
        upgrade.buy(&mut self.state);

        self.computed.recalc_cps(&self.state);
        self.computed.recalc_upgrades(&self.state);

        true
    }

    pub fn tick(&mut self) {
        self.state.tick(&self.computed);
        self.computed.tick(&self.state);
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

#[derive(Serialize, Deserialize, Debug)]
struct State {
    #[serde(default = "Cookies::new")]
    cookies: Cookies,
    #[serde(default = "Buildings::new")]
    buildings: Buildings,
    #[serde(default = "Milk::new")]
    milk: Milk,
    #[serde(default = "Achievements::new")]
    achievements: Achievements,
}

impl State {
    fn new() -> Self {
        Self {
            cookies: Cookies::new(),
            buildings: Buildings::new(),
            milk: Milk::new(),
            achievements: Achievements::new(),
        }
    }

    fn tick(&mut self, computed: &Computed) {
        self.cookies.tick(computed.cps);
        self.buildings.tick();
        self.milk.tick(self.achievements.owned().len() as _);

        achievement::tick(self, computed);
    }
}

#[derive(Debug)]
struct Computed {
    cps: f64,
    ticker: Ticker,
    upgrades: Upgrades,
}

impl Computed {
    fn new(state: &State) -> Self {
        let cps = self::calc::cps(state);
        let ticker = Ticker::new(state);
        let upgrades = Upgrades::new(state);

        Self {
            cps,
            ticker,
            upgrades,
        }
    }

    fn tick(&mut self, state: &State) {
        self.ticker.tick(state);
        self.upgrades.tick(state);
    }

    fn recalc_cps(&mut self, state: &State) {
        self.cps = self::calc::cps(state);
    }

    fn recalc_upgrades(&mut self, state: &State) {
        self.upgrades = Upgrades::new(state);
    }
}
