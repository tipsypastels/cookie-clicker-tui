use super::GrandmapocalypsePhase;
use crate::{calc, cookies::Cookies, cps::Cps};
use cookie_clicker_tui_utils::frames::FPS;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

const DEFAULT_MAX_SIZE: usize = 10;
#[cfg(debug_assertions)]
const DEFAULT_ODDS_PER_SPOT_PER_PHASE: f64 = 0.0001;
#[cfg(not(debug_assertions))]
const DEFAULT_ODDS_PER_SPOT_PER_PHASE: f64 = 0.00001;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wrinklers {
    list: Vec<Wrinkler>,
    max_size: usize,
    odds_per_spot_per_phase: f64,
    count_just_changed: bool,
}

impl Wrinklers {
    pub(crate) fn new() -> Self {
        Self {
            list: Vec::new(),
            max_size: DEFAULT_MAX_SIZE,
            odds_per_spot_per_phase: DEFAULT_ODDS_PER_SPOT_PER_PHASE,
            count_just_changed: false,
        }
    }

    pub(crate) fn tick(&mut self, phase: GrandmapocalypsePhase, cps: &Cps) {
        let available = self.available_size();
        if available > 0
            && rand::random::<f64>()
                <= self.odds_per_spot_per_phase * available as f64 * phase.wrinkler_spawn_mult()
        {
            self.list.push(Wrinkler::new());
            self.count_just_changed = true;
        } else {
            self.count_just_changed = false;
        }

        for wrinkler in &mut self.list {
            wrinkler.eat(cps);
        }
    }

    pub(crate) fn count_just_changed(&self) -> bool {
        self.count_just_changed
    }

    pub(crate) fn pop(&mut self, index: usize, cookies: &mut Cookies) {
        if let Some(wrinkler) = self.get(index) {
            let gain = calc::wrinkler_pop_cookies(wrinkler.eaten);
            cookies.gain_bulk(gain);
            self.list.remove(index);
        }
    }

    pub(crate) fn pop_all(&mut self, cookies: &mut Cookies) {
        let gain = self
            .list
            .iter()
            .map(|w| calc::wrinkler_pop_cookies(w.eaten))
            .sum();

        cookies.gain_bulk(gain);
        self.list.clear();
    }

    fn available_size(&self) -> usize {
        self.max_size - self.list.len()
    }
}

impl Deref for Wrinklers {
    type Target = [Wrinkler];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wrinkler {
    eaten: f64,
}

impl Wrinkler {
    fn new() -> Self {
        Self { eaten: 0.0 }
    }

    fn eat(&mut self, cps: &Cps) {
        self.eaten += cps.wrinkled / FPS;
    }

    pub fn eaten(&self) -> f64 {
        self.eaten
    }
}
