use super::state::State;
use crate::{event::FPS, num::big};
use rand::seq::IndexedRandom;
use std::ops::Range;

#[cfg(debug_assertions)]
const TICKS_PER_MESSAGE: u16 = 5 * FPS as u16;

#[cfg(not(debug_assertions))]
const TICKS_PER_MESSAGE: u16 = 30 * FPS as u16;

type Entry = (&'static str, Pred);

#[derive(Debug)]
enum Pred {
    CLe(u128),
    CGe(u128),
    CIn(Range<u128>),
}

impl Pred {
    fn matches(&self, state: &State) -> bool {
        match self {
            Self::CLe(n) => state.cookies < *n,
            Self::CGe(n) => state.cookies >= *n,
            Self::CIn(range) => state.cookies.in_range(range),
        }
    }
}

static ENTRIES: &[Entry] = &[
    (
        "You feel like making cookies. But nobody wants to eat your cookies.",
        Pred::CLe(5),
    ),
    (
        "Your first batch goes in the trash. The neighborhood raccoon barely touches it.",
        Pred::CIn(5..50),
    ),
    (
        "Your family accepts to try some of your cookies.",
        Pred::CIn(50..100),
    ),
    (
        "Your cookies are popular in the neighborhood.",
        Pred::CIn(100..500),
    ),
    (
        "People are starting to talk about your cookies.",
        Pred::CIn(500..big!(1 K)),
    ),
    (
        "Your cookies are talked about for miles around.",
        Pred::CIn(big!(1 K)..big!(5 K)),
    ),
    (
        "Your cookies are renowned in the whole town!",
        Pred::CIn(big!(5 K)..big!(10 K)),
    ),
    (
        "Your cookies bring all the boys to the yard.",
        Pred::CIn(big!(10 K)..big!(50 K)),
    ),
    (
        "Your cookies now have their own website!",
        Pred::CIn(big!(50 K)..big!(100 K)),
    ),
    (
        "Your cookies are worth a lot of money.",
        Pred::CIn(big!(100 K)..big!(500 K)),
    ),
    (
        "Your cookies sell very well in distant countries.",
        Pred::CIn(big!(500 K)..big!(1 M)),
    ),
    (
        "People come from very far away to get a taste of your cookies.",
        Pred::CIn(big!(1 M)..big!(5 M)),
    ),
    (
        "Kings and queens from all over the world are enjoying your cookies.",
        Pred::CIn(big!(5 M)..big!(10 M)),
    ),
    (
        "There are now museums dedicated to your cookies.",
        Pred::CIn(big!(10 M)..big!(50 M)),
    ),
    (
        "A national day has been created in honor of your cookies.",
        Pred::CIn(big!(50 M)..big!(100 M)),
    ),
    (
        "Your cookies have been named a part of the world wonders.",
        Pred::CIn(big!(100 M)..big!(500 M)),
    ),
    (
        "History books now include a whole chapter about your cookies.",
        Pred::CIn(big!(500 M)..big!(1 B)),
    ),
    (
        "Your cookies have been placed under government surveillance.",
        Pred::CIn(big!(1 B)..big!(5 B)),
    ),
    (
        "The whole planet is enjoying your cookies!",
        Pred::CIn(big!(5 B)..big!(10 B)),
    ),
    (
        "Strange creatures from neighboring planets wish to try your cookies.",
        Pred::CIn(big!(10 B)..big!(50 B)),
    ),
    (
        "Elder gods from the whole cosmos have awoken to taste your cookies.",
        Pred::CIn(big!(50 B)..big!(100 B)),
    ),
    (
        "Beings from other dimensions lapse into existence just to get a taste of your cookies.",
        Pred::CIn(big!(100 B)..big!(500 B)),
    ),
    (
        "Your cookies have achieved sentience.",
        Pred::CIn(big!(500 B)..big!(1 T)),
    ),
    (
        "The universe has now turned into cookie dough, to the molecular level.",
        Pred::CIn(big!(1 T)..big!(5 T)),
    ),
    (
        "Your cookies are rewriting the fundamental laws of the universe.",
        Pred::CIn(big!(5 T)..big!(10 T)),
    ),
    (
        "it's time to stop playing",
        Pred::CIn(big!(10 T)..big!(100 T)),
    ),
    (
        "A local news station runs a 10-minute segment about your cookies. Success!",
        Pred::CGe(big!(100 T)),
    ),
];

#[derive(Debug)]
pub struct Ticker {
    enabled_indices: Vec<usize>,
    current_index: Option<usize>,
    ticks_until_change: u16,
}

impl Ticker {
    pub fn new(state: &State) -> Self {
        let enabled_indices = get_enabled_indices(state);
        let current_index = enabled_indices.choose(&mut rand::rng()).copied();

        Self {
            enabled_indices,
            current_index,
            ticks_until_change: TICKS_PER_MESSAGE,
        }
    }

    pub fn text(&self) -> Option<&'static str> {
        self.current_index.and_then(|i| ENTRIES.get(i)).map(|e| e.0)
    }

    pub fn tick(&mut self, state: &State) {
        if let Some(ticks_until_change) = self.ticks_until_change.checked_sub(1) {
            self.ticks_until_change = ticks_until_change;
        } else {
            self.ticks_until_change = TICKS_PER_MESSAGE;
            self.enabled_indices = get_enabled_indices(state);
            self.current_index = self.enabled_indices.choose(&mut rand::rng()).copied();
        }
    }
}

fn get_enabled_indices(state: &State) -> Vec<usize> {
    ENTRIES
        .iter()
        .enumerate()
        .filter_map(|(i, (_, p))| p.matches(state).then_some(i))
        .collect()
}
