use super::state::State;
use crate::event::FPS;
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
        "You feel like making cookies. But nobody wants to eat your cookies. ",
        Pred::CLe(5),
    ),
    (
        "Your first batch goes in the trash. The neighborhood raccoon barely touches it.",
        Pred::CIn(5..50),
    ),
    (
        "Your family accepts to try some of your cookies. ",
        Pred::CIn(50..100),
    ),
    (
        "Your cookies are popular in the neighborhood. ",
        Pred::CIn(100..500),
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
