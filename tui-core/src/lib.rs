mod building;
mod requirement;
mod ticker;
mod upgrade;

use self::ticker::Ticker;

#[derive(Debug)]
pub struct Core {
    fps: f64,
    state: State,
    computed: Computed,
    ticker: Ticker,
}

#[derive(Debug)]
struct State {
    cookies: f64,
}

#[derive(Debug)]
struct Computed {
    cps: f64,
}

impl Core {
    pub fn new(fps: f64) -> Self {
        todo!()
    }

    pub fn cookies(&self) -> f64 {
        self.state.cookies
    }

    pub fn cps(&self) -> f64 {
        self.computed.cps
    }

    pub fn ticker(&self) -> Option<&'static str> {
        self.ticker.text()
    }

    pub fn tick(&mut self) {
        self.state.cookies += self.computed.cps / self.fps;
        self.ticker.tick(self.fps, &self.state, &self.computed);
    }
}
