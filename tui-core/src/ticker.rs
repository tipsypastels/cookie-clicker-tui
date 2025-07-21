use crate::{State, req::Req};
use cookie_clicker_tui_utils::{frames::RefreshClock, num};
use rand::seq::IndexedRandom;

static ENTRIES: &[(&str, Req)] = &[
    (
        "You feel like making cookies. But nobody wants to eat your cookies.",
        Req::CookiesBelow(5.0),
    ),
    (
        "Your first batch goes in the trash. The neighborhood raccoon barely touches it.",
        Req::CookiesRange(5.0, 50.0),
    ),
    (
        "Your family accepts to try some of your cookies.",
        Req::CookiesRange(50.0, 100.0),
    ),
    (
        "Your cookies are popular in the neighborhood.",
        Req::CookiesRange(100.0, 500.0),
    ),
    (
        "People are starting to talk about your cookies.",
        Req::CookiesRange(500.0, 1.0 * num::THOUSAND),
    ),
    (
        "Your cookies are talked about for miles around.",
        Req::CookiesRange(1.0 * num::THOUSAND, 5.0 * num::THOUSAND),
    ),
    (
        "Your cookies are renowned in the whole town!",
        Req::CookiesRange(5.0 * num::THOUSAND, 10.0 * num::THOUSAND),
    ),
    (
        "Your cookies bring all the boys to the yard.",
        Req::CookiesRange(10.0 * num::THOUSAND, 50.0 * num::THOUSAND),
    ),
    (
        "Your cookies now have their own website!",
        Req::CookiesRange(50.0 * num::THOUSAND, 100.0 * num::THOUSAND),
    ),
    (
        "Your cookies are worth a lot of money.",
        Req::CookiesRange(100.0 * num::THOUSAND, 500.0 * num::THOUSAND),
    ),
    (
        "Your cookies sell very well in distant countries.",
        Req::CookiesRange(500.0 * num::THOUSAND, 1.0 * num::MILLION),
    ),
    (
        "People come from very far away to get a taste of your cookies.",
        Req::CookiesRange(1.0 * num::MILLION, 5.0 * num::MILLION),
    ),
    (
        "Kings and queens from all over the world are enjoying your cookies.",
        Req::CookiesRange(5.0 * num::MILLION, 10.0 * num::MILLION),
    ),
    (
        "There are now museums dedicated to your cookies.",
        Req::CookiesRange(10.0 * num::MILLION, 50.0 * num::MILLION),
    ),
    (
        "A national day has been created in honor of your cookies.",
        Req::CookiesRange(50.0 * num::MILLION, 100.0 * num::MILLION),
    ),
    (
        "Your cookies have been named a part of the world wonders.",
        Req::CookiesRange(100.0 * num::MILLION, 500.0 * num::MILLION),
    ),
    (
        "History books now include a whole chapter about your cookies.",
        Req::CookiesRange(500.0 * num::MILLION, 1.0 * num::BILLION),
    ),
    (
        "Your cookies have been placed under government surveillance.",
        Req::CookiesRange(1.0 * num::BILLION, 5.0 * num::BILLION),
    ),
    (
        "The whole planet is enjoying your cookies!",
        Req::CookiesRange(5.0 * num::BILLION, 10.0 * num::BILLION),
    ),
    (
        "Strange creatures from neighboring planets wish to try your cookies.",
        Req::CookiesRange(10.0 * num::BILLION, 50.0 * num::BILLION),
    ),
    (
        "Elder gods from the whole cosmos have awoken to taste your cookies.",
        Req::CookiesRange(50.0 * num::BILLION, 100.0 * num::BILLION),
    ),
    (
        "Beings from other dimensions lapse into existence just to get a taste of your cookies.",
        Req::CookiesRange(100.0 * num::BILLION, 500.0 * num::BILLION),
    ),
    (
        "Your cookies have achieved sentience.",
        Req::CookiesRange(500.0 * num::BILLION, 1.0 * num::TRILLION),
    ),
    (
        "The universe has now turned into cookie dough, to the molecular level.",
        Req::CookiesRange(1.0 * num::TRILLION, 5.0 * num::TRILLION),
    ),
    (
        "Your cookies are rewriting the fundamental laws of the universe.",
        Req::CookiesRange(5.0 * num::TRILLION, 10.0 * num::TRILLION),
    ),
    (
        "it's time to stop playing",
        Req::CookiesRange(10.0 * num::TRILLION, 100.0 * num::TRILLION),
    ),
    (
        "A local news station runs a 10-minute segment about your cookies. Success!",
        Req::CookiesAboveOrEq(100.0 * num::TRILLION),
    ),
];

#[derive(Debug)]
pub struct Ticker {
    current_index: Option<usize>,
    refresh: RefreshClock<30>,
}

impl Ticker {
    pub fn new(state: &State) -> Self {
        let enabled_indices = get_indices(state);
        let current_index = enabled_indices.choose(&mut rand::rng()).copied();
        let refresh = RefreshClock::new();

        Self {
            current_index,
            refresh,
        }
    }

    pub fn text(&self) -> Option<&'static str> {
        self.current_index.and_then(|i| ENTRIES.get(i).map(|e| e.0))
    }

    pub fn tick(&mut self, state: &State) {
        if self.refresh.finish() {
            *self = Self::new(state);
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
