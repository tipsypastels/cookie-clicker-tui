use cookie_clicker_tui_utils::frames::FPS;
use serde::{Deserialize, Serialize};

use crate::macros::impl_serde_from_state;

#[derive(Debug)]
pub struct Spawner {
    state: SpawnerState,
    computed: SpawnerComputed,
}

impl Spawner {
    pub fn new(tmin_secs: f64, tmax_secs: f64) -> Self {
        Self::from_state(SpawnerState {
            tmin_secs,
            tmax_secs,
        })
    }

    fn from_state(state: SpawnerState) -> Self {
        let computed = SpawnerComputed::new(&state);
        Self { state, computed }
    }

    pub fn spawn(&mut self) -> bool {
        self.computed.spawn()
    }

    pub fn modify(&mut self, f: impl FnOnce(&mut f64, &mut f64)) {
        f(&mut self.state.tmin_secs, &mut self.state.tmax_secs);
        self.computed.tmin = self.state.tmin_secs * FPS;
        self.computed.tmax = self.state.tmax_secs * FPS;
    }
}

impl_serde_from_state!(Spawner as state: SpawnerState);

#[derive(Serialize, Deserialize, Debug)]
struct SpawnerState {
    tmin_secs: f64,
    tmax_secs: f64,
}

#[derive(Debug)]
struct SpawnerComputed {
    tmin: f64,
    tmax: f64,
    n: f64,
    #[cfg(debug_assertions)]
    _n_last_hit: Option<f64>,
    #[cfg(debug_assertions)]
    _prob: f64,
}

impl SpawnerComputed {
    fn new(state: &SpawnerState) -> Self {
        Self {
            tmin: state.tmin_secs * FPS,
            tmax: state.tmax_secs * FPS,
            n: 0.0,
            #[cfg(debug_assertions)]
            _n_last_hit: None,
            #[cfg(debug_assertions)]
            _prob: 0.0,
        }
    }

    fn spawn(&mut self) -> bool {
        let prob = f64::max(0.0, (self.n - self.tmin) / (self.tmax - self.tmin)).powi(5);

        #[cfg(debug_assertions)]
        {
            self._prob = prob;
        }

        if rand::random::<f64>() < prob {
            #[cfg(debug_assertions)]
            {
                self._n_last_hit = Some(self.n);
            }
            self.n = 0.0;
            true
        } else {
            self.n += 1.0;
            false
        }
    }
}
