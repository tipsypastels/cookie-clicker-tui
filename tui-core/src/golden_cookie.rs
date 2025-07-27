use crate::{macros, spawner::Spawner};
use cookie_clicker_tui_utils::refresh::Refresh;
use enum_assoc::Assoc;
use enum_fun::Variants;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DEFAULT_TMIN_SECS: f64 = 300.0;
const DEFAULT_TMAX_SECS: f64 = 900.0;
const DEFAULT_DURATION_SECS: f64 = 13.0;

#[derive(Debug)]
pub struct GoldenCookies {
    state: GoldenCookieState,
    list: GoldenCookieList,
}

impl GoldenCookies {
    pub(crate) fn new() -> Self {
        Self::from_state(GoldenCookieState::new())
    }

    fn from_state(state: GoldenCookieState) -> Self {
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
        let Some(ch) = GoldenCookieInputChar::from_char(ch) else {
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
macros::deserialize_via_state!(GoldenCookies => GoldenCookieState as GoldenCookies::from_state);

#[derive(Serialize, Deserialize, Debug)]
struct GoldenCookieState {
    click_count: usize,
    click_miss_count: usize,
    clicked_one_at_most_1s_after_spawn: bool,
    clicked_one_at_most_1s_before_despawn: bool,
    cookie_duration_secs: f64,
    spawner: Spawner,
}

impl GoldenCookieState {
    fn new() -> Self {
        Self {
            click_count: 0,
            click_miss_count: 0,
            clicked_one_at_most_1s_after_spawn: false,
            clicked_one_at_most_1s_before_despawn: false,
            cookie_duration_secs: DEFAULT_DURATION_SECS,
            spawner: Spawner::new(DEFAULT_TMIN_SECS, DEFAULT_TMAX_SECS),
        }
    }
}

#[derive(Debug)]
struct GoldenCookieList {
    map: HashMap<GoldenCookieInputChar, GoldenCookie>,
}

impl GoldenCookieList {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn spawn(&mut self, duration_secs: f64) {
        let available = GoldenCookieInputChar::variants()
            .filter(|ch| !self.map.contains_key(ch))
            .collect::<Vec<_>>();

        let Some(ch) = available.choose(&mut rand::rng()).copied() else {
            return;
        };
        self.map.insert(ch, GoldenCookie::new(ch, duration_secs));
    }

    fn remove_and_count_missed(&mut self) -> usize {
        self.map
            .extract_if(|_, cookie| cookie.refresh.finish())
            .count()
    }
}

#[derive(Debug)]
pub struct GoldenCookie {
    ch: GoldenCookieInputChar,
    x: f64,
    y: f64,
    refresh: Refresh,
}

impl GoldenCookie {
    fn new(ch: GoldenCookieInputChar, duration_secs: f64) -> Self {
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

#[derive(Assoc, Variants, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(const fn char(self) -> char)]
#[func(const fn from_char(ch: char) -> Option<Self>)]
enum GoldenCookieInputChar {
    #[assoc(char = '1', from_char = '1')]
    One,
    #[assoc(char = '2', from_char = '2')]
    Two,
    #[assoc(char = '3', from_char = '3')]
    Three,
    #[assoc(char = '4', from_char = '4')]
    Four,
    #[assoc(char = '5', from_char = '5')]
    Five,
    #[assoc(char = '6', from_char = '6')]
    Six,
    #[assoc(char = '7', from_char = '7')]
    Seven,
    #[assoc(char = '8', from_char = '8')]
    Eight,
    #[assoc(char = '9', from_char = '9')]
    Nine,
}
