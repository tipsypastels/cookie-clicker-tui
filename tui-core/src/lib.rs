mod building;
mod upgrade;

#[derive(Debug)]
pub struct Core {
    state: State,
    cached: Cached,
}

#[derive(Debug)]
struct State {}

#[derive(Debug)]
struct Cached {
    cps: f64,
}

impl Core {
    pub fn new() -> Self {
        todo!()
    }

    pub fn cps(&self) -> f64 {
        self.cached.cps
    }

    pub fn tick(&mut self, fps: f64) {}
}
