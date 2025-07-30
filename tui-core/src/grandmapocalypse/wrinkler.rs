use super::GrandmapocalypsePhase;
use crate::{Changeset, calc, cookies::Cookies, cps::Cps};
use cookie_clicker_tui_utils::frames::FPS;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

const SHINY_ODDS: f64 = 0.0001;

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
    pop_count: usize,
    popped_shiny_ever: bool,
}

impl Wrinklers {
    pub(crate) fn new() -> Self {
        Self {
            list: Vec::new(),
            max_size: DEFAULT_MAX_SIZE,
            odds_per_spot_per_phase: DEFAULT_ODDS_PER_SPOT_PER_PHASE,
            pop_count: 0,
            popped_shiny_ever: false,
        }
    }

    pub(crate) fn tick(
        &mut self,
        phase: GrandmapocalypsePhase,
        cps: &Cps,
        changeset: &mut Changeset,
    ) {
        let available = self.available_size();
        if available > 0
            && rand::random::<f64>()
                <= self.odds_per_spot_per_phase * available as f64 * phase.wrinkler_spawn_mult()
        {
            self.list.push(Wrinkler::new());
            changeset.cps = true;
        }

        for wrinkler in &mut self.list {
            wrinkler.eat(cps);
        }
    }

    pub(crate) fn pop_count(&self) -> usize {
        self.pop_count
    }

    pub(crate) fn popped_shiny_ever(&self) -> bool {
        self.popped_shiny_ever
    }

    pub(crate) fn pop(&mut self, index: usize, cookies: &mut Cookies, changeset: &mut Changeset) {
        if let Some(wrinkler) = self.get(index) {
            let gain = calc::wrinkler_pop_cookies(wrinkler.eaten, wrinkler.shiny);

            cookies.gain_bulk(gain);
            changeset.cps = true;

            self.popped_shiny_ever |= wrinkler.shiny;
            self.pop_count = self.pop_count.saturating_add(1);
            self.list.remove(index);
        }
    }

    pub(crate) fn pop_all(&mut self, cookies: &mut Cookies, changeset: &mut Changeset) {
        let mut gain = 0.0;
        let mut shiny = false;

        for wrinkler in self.list.iter() {
            gain += calc::wrinkler_pop_cookies(wrinkler.eaten, wrinkler.shiny);
            shiny |= wrinkler.shiny;
        }

        cookies.gain_bulk(gain);
        changeset.cps = true;

        self.popped_shiny_ever |= shiny;
        self.pop_count = self.pop_count.saturating_add(1);
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
    shiny: bool,
}

impl Wrinkler {
    fn new() -> Self {
        Self {
            eaten: 0.0,
            shiny: rand::random::<f64>() <= SHINY_ODDS,
        }
    }

    fn eat(&mut self, cps: &Cps) {
        self.eaten += cps.wrinkled / FPS;
    }

    pub fn eaten(&self) -> f64 {
        self.eaten
    }

    pub fn shiny(&self) -> bool {
        self.shiny
    }
}
