use crate::{Computed, State, requirement::Requirement};
use cookie_clicker_tui_utils::num;
use rand::seq::IndexedRandom;

const SECONDS_UNTIL_CHANGE: f64 = 30.0;

static ENTRIES: &[(&str, Requirement)] = &[
    (
        "You feel like making cookies. But nobody wants to eat your cookies.",
        Requirement::cookies_below(5.0),
    ),
    (
        "Your first batch goes in the trash. The neighborhood raccoon barely touches it.",
        Requirement::cookies_range(5.0, 50.0),
    ),
    (
        "Your family accepts to try some of your cookies.",
        Requirement::cookies_range(50.0, 100.0),
    ),
    (
        "Your cookies are popular in the neighborhood.",
        Requirement::cookies_range(100.0, 500.0),
    ),
    (
        "People are starting to talk about your cookies.",
        Requirement::cookies_range(500.0, 1.0 * num::THOUSAND),
    ),
    (
        "Your cookies are talked about for miles around.",
        Requirement::cookies_range(1.0 * num::THOUSAND, 5.0 * num::THOUSAND),
    ),
    (
        "Your cookies are renowned in the whole town!",
        Requirement::cookies_range(5.0 * num::THOUSAND, 10.0 * num::THOUSAND),
    ),
    (
        "Your cookies bring all the boys to the yard.",
        Requirement::cookies_range(10.0 * num::THOUSAND, 50.0 * num::THOUSAND),
    ),
    (
        "Your cookies now have their own website!",
        Requirement::cookies_range(50.0 * num::THOUSAND, 100.0 * num::THOUSAND),
    ),
    (
        "Your cookies are worth a lot of money.",
        Requirement::cookies_range(100.0 * num::THOUSAND, 500.0 * num::THOUSAND),
    ),
    (
        "Your cookies sell very well in distant countries.",
        Requirement::cookies_range(500.0 * num::THOUSAND, 1.0 * num::MILLION),
    ),
    (
        "People come from very far away to get a taste of your cookies.",
        Requirement::cookies_range(1.0 * num::MILLION, 5.0 * num::MILLION),
    ),
    (
        "Kings and queens from all over the world are enjoying your cookies.",
        Requirement::cookies_range(5.0 * num::MILLION, 10.0 * num::MILLION),
    ),
    (
        "There are now museums dedicated to your cookies.",
        Requirement::cookies_range(10.0 * num::MILLION, 50.0 * num::MILLION),
    ),
    (
        "A national day has been created in honor of your cookies.",
        Requirement::cookies_range(50.0 * num::MILLION, 100.0 * num::MILLION),
    ),
    (
        "Your cookies have been named a part of the world wonders.",
        Requirement::cookies_range(100.0 * num::MILLION, 500.0 * num::MILLION),
    ),
    (
        "History books now include a whole chapter about your cookies.",
        Requirement::cookies_range(500.0 * num::MILLION, 1.0 * num::BILLION),
    ),
    (
        "Your cookies have been placed under government surveillance.",
        Requirement::cookies_range(1.0 * num::BILLION, 5.0 * num::BILLION),
    ),
    (
        "The whole planet is enjoying your cookies!",
        Requirement::cookies_range(5.0 * num::BILLION, 10.0 * num::BILLION),
    ),
    (
        "Strange creatures from neighboring planets wish to try your cookies.",
        Requirement::cookies_range(10.0 * num::BILLION, 50.0 * num::BILLION),
    ),
    (
        "Elder gods from the whole cosmos have awoken to taste your cookies.",
        Requirement::cookies_range(50.0 * num::BILLION, 100.0 * num::BILLION),
    ),
    (
        "Beings from other dimensions lapse into existence just to get a taste of your cookies.",
        Requirement::cookies_range(100.0 * num::BILLION, 500.0 * num::BILLION),
    ),
    (
        "Your cookies have achieved sentience.",
        Requirement::cookies_range(500.0 * num::BILLION, 1.0 * num::TRILLION),
    ),
    (
        "The universe has now turned into cookie dough, to the molecular level.",
        Requirement::cookies_range(1.0 * num::TRILLION, 5.0 * num::TRILLION),
    ),
    (
        "Your cookies are rewriting the fundamental laws of the universe.",
        Requirement::cookies_range(5.0 * num::TRILLION, 10.0 * num::TRILLION),
    ),
    (
        "it's time to stop playing",
        Requirement::cookies_range(10.0 * num::TRILLION, 100.0 * num::TRILLION),
    ),
    (
        "A local news station runs a 10-minute segment about your cookies. Success!",
        Requirement::cookies_above_or_eq(100.0 * num::TRILLION),
    ),
];

#[derive(Debug)]
pub struct Ticker {
    current_index: Option<usize>,
    ticks_until_change: u16,
}

impl Ticker {
    pub fn new(fps: f64, state: &State) -> Self {
        let enabled_indices = get_indices(state);
        let current_index = enabled_indices.choose(&mut rand::rng()).copied();
        let ticks_until_change = (SECONDS_UNTIL_CHANGE * fps) as u16;

        Self {
            current_index,
            ticks_until_change,
        }
    }

    pub fn text(&self) -> Option<&'static str> {
        self.current_index.and_then(|i| ENTRIES.get(i).map(|e| e.0))
    }

    pub fn tick(&mut self, fps: f64, state: &State) {
        if let Some(ticks_until_change) = self.ticks_until_change.checked_sub(1) {
            self.ticks_until_change = ticks_until_change;
        } else {
            *self = Self::new(fps, state);
        }
    }
}

fn get_indices(state: &State) -> Vec<usize> {
    ENTRIES
        .iter()
        .enumerate()
        .filter_map(|(i, (_, p))| p.check(state).then_some(i))
        .collect()
}
