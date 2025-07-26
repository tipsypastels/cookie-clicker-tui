use cookie_clicker_tui_utils::frames::FPS;

#[derive(Debug)]
pub struct Spawner {
    tmin_secs: f64,
    tmax_secs: f64,
    tmin: f64,
    tmax: f64,
    n: f64,
}

impl Spawner {
    pub fn new(tmin_secs: f64, tmax_secs: f64) -> Self {
        Self {
            tmin_secs,
            tmax_secs,
            tmin: tmin_secs * FPS,
            tmax: tmax_secs * FPS,
            n: tmin_secs * FPS,
        }
    }

    pub fn spawn(&mut self) -> bool {
        self.spawn_and_report_prob().0
    }

    pub fn spawn_and_report_prob(&mut self) -> (bool, f64) {
        let prob = f64::max(0.0, (self.n - self.tmin) / (self.tmax - self.tmin)).powi(5);
        let spawned = if rand::random::<f64>() < prob {
            self.n = 0.0;
            true
        } else {
            self.n += 1.0;
            false
        };
        (spawned, prob)
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut f64, &mut f64)) {
        f(&mut self.tmin_secs, &mut self.tmax_secs);
        self.tmin = self.tmin_secs * FPS;
        self.tmax = self.tmax_secs * FPS;
    }
}
