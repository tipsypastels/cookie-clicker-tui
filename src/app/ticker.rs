use super::state::State;
use crate::{event::FPS, num};
use rand::seq::IndexedRandom;
use std::ops::Range;

#[cfg(debug_assertions)]
const TICKS_PER_MESSAGE: u16 = 5 * FPS as u16;

#[cfg(not(debug_assertions))]
const TICKS_PER_MESSAGE: u16 = 30 * FPS as u16;

type Entry = (&'static str, Pred);

#[derive(Debug)]
enum Pred {
    CLe(f64),
    CGe(f64),
    CIn(Range<f64>),
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
        Pred::CLe(5.0),
    ),
    (
        "Your first batch goes in the trash. The neighborhood raccoon barely touches it.",
        Pred::CIn(5.0..50.0),
    ),
    (
        "Your family accepts to try some of your cookies.",
        Pred::CIn(50.0..100.0),
    ),
    (
        "Your cookies are popular in the neighborhood.",
        Pred::CIn(100.0..500.0),
    ),
    (
        "People are starting to talk about your cookies.",
        Pred::CIn(500.0..(1.0 * num::THOUSAND)),
    ),
    (
        "Your cookies are talked about for miles around.",
        Pred::CIn((1.0 * num::THOUSAND)..(5.0 * num::THOUSAND)),
    ),
    (
        "Your cookies are renowned in the whole town!",
        Pred::CIn((5.0 * num::THOUSAND)..(10.0 * num::THOUSAND)),
    ),
    (
        "Your cookies bring all the boys to the yard.",
        Pred::CIn((10.0 * num::THOUSAND)..(50.0 * num::THOUSAND)),
    ),
    (
        "Your cookies now have their own website!",
        Pred::CIn((50.0 * num::THOUSAND)..(100.0 * num::THOUSAND)),
    ),
    (
        "Your cookies are worth a lot of money.",
        Pred::CIn((100.0 * num::THOUSAND)..(500.0 * num::THOUSAND)),
    ),
    (
        "Your cookies sell very well in distant countries.",
        Pred::CIn((500.0 * num::THOUSAND)..(1.0 * num::MILLION)),
    ),
    (
        "People come from very far away to get a taste of your cookies.",
        Pred::CIn((1.0 * num::MILLION)..(5.0 * num::MILLION)),
    ),
    (
        "Kings and queens from all over the world are enjoying your cookies.",
        Pred::CIn((5.0 * num::MILLION)..(10.0 * num::MILLION)),
    ),
    (
        "There are now museums dedicated to your cookies.",
        Pred::CIn((10.0 * num::MILLION)..(50.0 * num::MILLION)),
    ),
    (
        "A national day has been created in honor of your cookies.",
        Pred::CIn((50.0 * num::MILLION)..(100.0 * num::MILLION)),
    ),
    (
        "Your cookies have been named a part of the world wonders.",
        Pred::CIn((100.0 * num::MILLION)..(500.0 * num::MILLION)),
    ),
    (
        "History books now include a whole chapter about your cookies.",
        Pred::CIn((500.0 * num::MILLION)..(1.0 * num::BILLION)),
    ),
    (
        "Your cookies have been placed under government surveillance.",
        Pred::CIn((1.0 * num::BILLION)..(5.0 * num::BILLION)),
    ),
    (
        "The whole planet is enjoying your cookies!",
        Pred::CIn((5.0 * num::BILLION)..(10.0 * num::BILLION)),
    ),
    (
        "Strange creatures from neighboring planets wish to try your cookies.",
        Pred::CIn((10.0 * num::BILLION)..(50.0 * num::BILLION)),
    ),
    (
        "Elder gods from the whole cosmos have awoken to taste your cookies.",
        Pred::CIn((50.0 * num::BILLION)..(100.0 * num::BILLION)),
    ),
    (
        "Beings from other dimensions lapse into existence just to get a taste of your cookies.",
        Pred::CIn((100.0 * num::BILLION)..(500.0 * num::BILLION)),
    ),
    (
        "Your cookies have achieved sentience.",
        Pred::CIn((500.0 * num::BILLION)..(1.0 * num::TRILLION)),
    ),
    (
        "The universe has now turned into cookie dough, to the molecular level.",
        Pred::CIn((1.0 * num::TRILLION)..(5.0 * num::TRILLION)),
    ),
    (
        "Your cookies are rewriting the fundamental laws of the universe.",
        Pred::CIn((5.0 * num::TRILLION)..(10.0 * num::TRILLION)),
    ),
    (
        "it's time to stop playing",
        Pred::CIn((10.0 * num::TRILLION)..(100.0 * num::TRILLION)),
    ),
    (
        "A local news station runs a 10-minute segment about your cookies. Success!",
        Pred::CGe(100.0 * num::TRILLION),
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
