use crate::spawner::Spawner;
use enum_assoc::Assoc;
use enum_fun::Variants;
use rand::seq::IndexedRandom;
use std::collections::HashMap;

const DEFAULT_TMIN_SECS: f64 = 300.0;
const DEFAULT_TMAX_SECS: f64 = 900.0;

#[derive(Debug)]
pub struct GoldenCookies {
    list: GoldenCookieList,
    spawner: Spawner,
}

impl GoldenCookies {
    pub(crate) fn new() -> Self {
        Self {
            list: GoldenCookieList::new(),
            spawner: Spawner::new(DEFAULT_TMIN_SECS, DEFAULT_TMAX_SECS),
        }
    }

    pub(crate) fn tick(&mut self) {
        if self.spawner.spawn() {
            self.list.spawn();
        }
    }

    pub(crate) fn click(&mut self, ch: char) -> bool {
        let Some(ch) = GoldenCookieInputChar::from_char(ch) else {
            return false;
        };
        let Some(_cookie) = self.list.map.remove(&ch) else {
            return false;
        };

        true
    }

    pub(crate) fn modify_spawning(&mut self, f: impl FnOnce(&mut f64, &mut f64)) {
        self.spawner.modify(f);
    }

    pub fn iter(&self) -> impl Iterator<Item = &GoldenCookie> {
        self.list.map.values()
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

    fn spawn(&mut self) {
        let available = GoldenCookieInputChar::variants()
            .filter(|ch| !self.map.contains_key(ch))
            .collect::<Vec<_>>();

        let Some(ch) = available.choose(&mut rand::rng()).copied() else {
            return;
        };

        self.map.insert(ch, GoldenCookie::new(ch));
    }
}

#[derive(Debug)]
pub struct GoldenCookie {
    ch: GoldenCookieInputChar,
    x: f64,
    y: f64,
}

impl GoldenCookie {
    fn new(ch: GoldenCookieInputChar) -> Self {
        Self {
            ch,
            x: rand::random(),
            y: rand::random(),
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
