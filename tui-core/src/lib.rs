mod building;
mod calc;
mod requirement;
mod ticker;
mod upgrade;

pub use self::building::{Building, BuildingInfo};

use self::{building::Buildings, ticker::Ticker};

#[derive(Debug)]
pub struct Core {
    fps: f64,
    state: State,
    computed: Computed,
    ticker: Ticker,
}

impl Core {
    pub fn new(fps: f64) -> Self {
        let state = State::new();
        let computed = Computed::new(&state);
        let ticker = Ticker::new(fps, &state, &computed);

        Self {
            fps,
            state,
            computed,
            ticker,
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

    pub fn ticker(&self) -> Option<&'static str> {
        self.ticker.text()
    }

    pub fn tick(&mut self) {
        self.state.cookies += self.computed.cps / self.fps;
        self.ticker.tick(self.fps, &self.state, &self.computed);
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
}

impl Computed {
    fn new(state: &State) -> Self {
        let cps = self::calc::cps(state);
        Self { cps }
    }
}
