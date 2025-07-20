mod achivement;
mod building;
mod calc;
mod req;
mod ticker;
mod upgrade;

pub use self::{
    achivement::Achivement,
    building::{Building, BuildingInfo},
    upgrade::{Upgrade, UpgradeEffectInfo},
};

use self::{achivement::Achivements, building::Buildings, ticker::Ticker, upgrade::Upgrades};
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Core {
    fps: f64,
    state: State,
    computed: Computed,
    late_computed: LateComputed,
}

impl Core {
    pub fn new(fps: f64) -> Self {
        let state = State::new();
        let computed = Computed::new(fps, &state);
        let late_computed = LateComputed::new(fps);

        Self {
            fps,
            state,
            computed,
            late_computed,
        }
    }

    pub fn cookies(&self) -> f64 {
        self.state.cookies
    }

    pub fn cps(&self) -> f64 {
        self.computed.cps
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

    pub fn owned_achivements(&self) -> &BTreeSet<Achivement> {
        self.late_computed.achivements.owned()
    }

    pub fn queued_achivement(&self) -> Option<Achivement> {
        self.late_computed.achivements.queued()
    }

    pub fn ticker(&self) -> Option<&'static str> {
        self.computed.ticker.text()
    }

    pub fn give_cookies(&mut self, amount: f64) {
        self.state.cookies += amount;
    }

    pub fn buy_building(&mut self, building: Building) -> bool {
        let cost = self.building_info(building).cost();

        if cost > self.state.cookies {
            return false;
        }

        self.state.cookies -= cost;
        self.state.buildings.modify(building, |b| b.count += 1);

        self.computed.recalc_cps(&self.state);
        self.computed.recalc_upgrades(self.fps, &self.state);

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
        self.computed.recalc_upgrades(self.fps, &self.state);

        true
    }

    pub fn tick(&mut self) {
        self.state.cookies += self.computed.cps / self.fps;
        self.computed.tick(self.fps, &self.state);
        self.late_computed
            .tick(self.fps, &self.state, &self.computed);
    }
}

#[derive(Debug)]
struct State {
    cookies: f64,
    buildings: Buildings,
}

impl State {
    fn new() -> Self {
        Self {
            cookies: 0.0,
            buildings: Buildings::new(),
        }
    }
}

#[derive(Debug)]
struct Computed {
    cps: f64,
    ticker: Ticker,
    upgrades: Upgrades,
}

impl Computed {
    fn new(fps: f64, state: &State) -> Self {
        let cps = self::calc::cps(state);
        let ticker = Ticker::new(fps, state);
        let upgrades = Upgrades::new(fps, state);

        Self {
            cps,
            ticker,
            upgrades,
        }
    }

    fn tick(&mut self, fps: f64, state: &State) {
        self.ticker.tick(fps, state);
        self.upgrades.tick(fps, state);
    }

    fn recalc_cps(&mut self, state: &State) {
        self.cps = self::calc::cps(state);
    }

    fn recalc_upgrades(&mut self, fps: f64, state: &State) {
        self.upgrades = Upgrades::new(fps, state);
    }
}

#[derive(Debug)]
struct LateComputed {
    achivements: Achivements,
}

impl LateComputed {
    fn new(fps: f64) -> Self {
        let achivements = Achivements::new(fps);

        Self { achivements }
    }

    fn tick(&mut self, fps: f64, state: &State, computed: &Computed) {
        self.achivements.tick(fps, state, computed);
    }
}
