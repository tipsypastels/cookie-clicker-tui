use crate::{Building, Changeset, building::Buildings, calc, macros};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Click {
    state: ClickState,
    cpc: f64,
    non_cursor_buildings_count: Option<u16>,
}

impl Click {
    pub fn new() -> Self {
        Self::from_state(ClickState::default())
    }

    fn from_state(state: ClickState) -> Self {
        Self {
            state,
            cpc: 1.0,
            non_cursor_buildings_count: None,
        }
    }

    pub fn tick(&mut self, buildings: &Buildings, changeset: &Changeset) {
        if changeset.buildings_count || self.non_cursor_buildings_count.is_none() {
            self.non_cursor_buildings_count =
                Some(buildings.total_count() - buildings.count(Building::Cursor));
        }
    }

    pub fn cpc(&self) -> f64 {
        self.cpc
    }

    pub fn set_thousand_fingers_mult(&mut self, mult: Option<f64>) {
        self.state.thousand_fingers_mult = mult;
        self.recalc_cpc();
    }

    fn recalc_cpc(&mut self) {
        self.cpc = calc::cpc(
            self.non_cursor_buildings_count
                .zip(self.state.thousand_fingers_mult),
        );
    }
}

macros::serialize_via_state!(Click => CursorState as |c| c.state);
macros::deserialize_via_state!(Click => ClickState as Click::from_state);

#[derive(Serialize, Deserialize, Default, Debug)]
struct ClickState {
    thousand_fingers_mult: Option<f64>,
}
