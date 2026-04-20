mod effect;
mod input;
mod list;
mod spawn;
mod stats;

use self::{input::InputChar, list::List, spawn::SpawningMode, stats::Stats};
use crate::{macros, spawner::Spawner};
use cookie_clicker_tui_utils::refresh::Refresh;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DEFAULT_DURATION_SECS: f64 = 13.0;

#[derive(Debug)]
pub struct GoldenCookies {
    state: State,
    list: List,
}

impl GoldenCookies {
    pub(crate) fn new() -> Self {
        Self::from_state(State::new())
    }

    fn from_state(state: State) -> Self {
        Self {
            state,
            list: GoldenCookieList::new(),
        }
    }

    pub(crate) fn tick(&mut self) {
        let missed = self.list.remove_and_count_missed();
        self.state.click_miss_count = self.state.click_miss_count.saturating_add(missed);

        if self.state.spawner.spawn() {
            self.list.spawn(self.state.cookie_duration_secs);
        }
    }

    pub(crate) fn click(&mut self, ch: char) -> bool {
        let Some(ch) = InputChar::from_char(ch) else {
            return false;
        };
        let Some(cookie) = self.list.map.remove(&ch) else {
            return false;
        };

        if cookie.refresh.cur_secs() <= 1.0 {
            self.state.clicked_one_at_most_1s_after_spawn = true;
        }
        if cookie.refresh.until_finish_secs() <= 1.0 {
            self.state.clicked_one_at_most_1s_before_despawn = true;
        }

        self.state.click_count = self.state.click_count.saturating_add(1);
        true
    }

    pub(crate) fn modify_spawning(&mut self, f: impl FnOnce(&mut f64, &mut f64)) {
        self.state.spawner.modify(f);
    }

    pub fn click_count(&self) -> usize {
        self.state.click_count
    }

    pub fn click_miss_count(&self) -> usize {
        self.state.click_miss_count
    }

    pub fn clicked_one_at_most_1s_after_spawn(&self) -> bool {
        self.state.clicked_one_at_most_1s_after_spawn
    }

    pub fn clicked_one_at_most_1s_before_despawn(&self) -> bool {
        self.state.clicked_one_at_most_1s_before_despawn
    }

    pub fn iter(&self) -> impl Iterator<Item = &GoldenCookie> {
        self.list.map.values()
    }
}

macros::serialize_via_state!(GoldenCookies => GoldenCookieState as |gc| gc.state);
macros::deserialize_via_state!(GoldenCookies => State as GoldenCookies::from_state);

#[derive(Serialize, Deserialize, Debug)]
struct State {
    stats: Stats,
    mode: SpawningMode,
}

impl State {
    fn new() -> Self {
        Self {
            stats: Stats::default(),
            mode: SpawningMode::default(),
        }
    }
}

#[derive(Debug)]
pub struct GoldenCookie {
    ch: InputChar,
    x: f64,
    y: f64,
    refresh: Refresh,
}

impl GoldenCookie {
    fn new(ch: InputChar, duration_secs: f64) -> Self {
        Self {
            ch,
            x: rand::random(),
            y: rand::random(),
            refresh: Refresh::new(duration_secs),
        }
    }

    pub fn ch(&self) -> char {
        self.ch.char()
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
