mod building;
mod calc;
mod requirement;
mod ticker;
mod upgrade;

pub use self::{
    building::{Building, BuildingInfo},
    upgrade::Upgrade,
};

use self::{building::Buildings, ticker::Ticker, upgrade::Upgrades};

#[derive(Debug)]
pub struct Core {
    fps: f64,
    state: State,
    computed: Computed,
}

impl Core {
    pub fn new(fps: f64) -> Self {
        let state = State::new();
        let computed = Computed::new(fps, &state);

        Self {
            fps,
            state,
            computed,
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

    pub fn unlocked_upgrades(&self) -> &[Upgrade] {
        &self.computed.upgrades
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

        true
    }

    pub fn tick(&mut self) {
        self.state.cookies += self.computed.cps / self.fps;
        self.computed.upgrades.tick(self.fps, &self.state);
        self.computed.ticker.tick(self.fps, &self.state);
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

    fn recalc_cps(&mut self, state: &State) {
        self.cps = self::calc::cps(state);
    }
}
