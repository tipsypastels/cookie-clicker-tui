use super::{GoldenCookie, input::InputChar};
use rand::seq::IndexedRandom;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct List {
    map: HashMap<InputChar, GoldenCookie>,
}

impl List {
    pub fn spawn(&mut self, duration_secs: f64) {
        let available = InputChar::variants()
            .filter(|ch| !self.map.contains_key(ch))
            .collect::<Vec<_>>();

        let Some(ch) = available.choose(&mut rand::rng()).copied() else {
            return;
        };
        self.map.insert(ch, GoldenCookie::new(ch, duration_secs));
    }

    pub fn remove_and_count_missed(&mut self) -> usize {
        self.map
            .extract_if(|_, cookie| cookie.refresh.finish())
            .count()
    }
}
