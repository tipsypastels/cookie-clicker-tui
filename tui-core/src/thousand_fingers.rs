use crate::{Changeset, building::Buildings, click::Click};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThousandFingers {
    enabled: bool,
    mult: f64,
}

impl ThousandFingers {
    pub fn new() -> Self {
        Self {
            enabled: false,
            mult: 1.0,
        }
    }

    pub fn mult(&self) -> Option<f64> {
        self.enabled.then_some(self.mult)
    }

    pub fn enable(
        &mut self,
        buildings: &mut Buildings,
        click: &mut Click,
        changeset: &mut Changeset,
    ) {
        self.enabled = true;
        self.apply(buildings, click, changeset);
    }

    pub fn multiply(
        &mut self,
        mult: f64,
        buildings: &mut Buildings,
        click: &mut Click,
        changeset: &mut Changeset,
    ) {
        self.mult *= mult;
        self.apply(buildings, click, changeset);
    }

    fn apply(&self, buildings: &mut Buildings, click: &mut Click, changeset: &mut Changeset) {
        buildings.set_thousand_fingers_mult(self.mult(), changeset);
        click.set_thousand_fingers_mult(self.mult());
    }
}
