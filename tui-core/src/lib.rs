mod achievement;
mod building;
mod calc;
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

use self::{achievement::Achievements, building::Buildings, ticker::Ticker, upgrade::Upgrades};
use cookie_clicker_tui_utils::frames::FPS;
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
        self.state.cookies
    }

    pub fn cookies_all_time(&self) -> f64 {
        self.state.cookies_all_time
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

    pub fn give_cookies(&mut self, amount: f64) {
        self.state.cookies += amount;
        self.state.cookies_all_time += amount;
    }

    pub fn buy_building(&mut self, building: Building) -> bool {
        let cost = self.building_info(building).cost();

        if cost > self.state.cookies {
            return false;
        }

        self.state.cookies -= cost;
        self.state.buildings.modify(building, |b| b.count += 1);

        self.computed.recalc_cps(&self.state);
        self.computed.recalc_upgrades(&self.state);

        true
    }

    pub fn buy_upgrade(&mut self, index: usize) -> bool {
        let Some(upgrade) = self.computed.upgrades.get(index) else {
            return false;
        };

        let cost = upgrade.cost();

        if cost > self.state.cookies {
            return false;
        }

        self.state.cookies -= cost;
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
    cookies: f64,
    cookies_all_time: f64,
    buildings: Buildings,
    milk: Milk,
    achievements: Achievements,
}

impl State {
    fn new() -> Self {
        Self {
            cookies: 0.0,
            cookies_all_time: 0.0,
            buildings: Buildings::new(),
            milk: Milk::new(),
            achievements: Achievements::new(),
        }
    }

    fn tick(&mut self, computed: &Computed) {
        let addl_cookies = computed.cps / FPS;

        self.cookies += addl_cookies;
        self.cookies_all_time += addl_cookies;

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
