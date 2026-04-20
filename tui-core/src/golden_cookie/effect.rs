pub mod frenzy;
pub mod lucky;

use self::{frenzy::Frenzy, lucky::Lucky};
use crate::{Changeset, cookies::Cookies, cps::Cps, macros};
use cookie_clicker_tui_utils::refresh::Refresh;
use rand::seq::IndexedRandom;

pub fn spawn() -> Effect {
    #[derive(Copy, Clone)]
    enum EffectKind {
        Lucky,
        Frenzy,
    }

    impl EffectKind {
        pub fn make(self) -> Effect {
            match self {
                Self::Lucky => Effect::Lucky(Lucky),
                Self::Frenzy => Effect::Frenzy(Frenzy),
            }
        }
    }

    let pool = [EffectKind::Lucky, EffectKind::Frenzy];
    let kind = pool.choose(&mut rand::rng()).unwrap();

    kind.make()
}

#[derive(Debug)]
pub enum Effect {
    Lucky(Lucky),
    Frenzy(Frenzy),
}

impl Effect {
    pub fn effect(
        self,
        cps: &Cps,
        cookies: &mut Cookies,
        clocks: &mut EffectClocks,
        changeset: &mut Changeset,
    ) {
    }
}

trait EffectImpl {
    fn effect(
        self,
        cps: &Cps,
        cookies: &mut Cookies,
        clocks: &mut EffectClocks,
        changeset: &mut Changeset,
    );
}

#[derive(Default, Debug)]
pub struct EffectClocks {
    duration_time_mult: Option<f64>,
    frenzy: Option<Refresh>,
}

impl EffectClocks {
    fn from_duration_time_mult(duration_time_mult: Option<f64>) -> Self {
        Self {
            duration_time_mult,
            ..Default::default()
        }
    }

    fn start(&mut self, time: f64, f: impl FnOnce(&mut EffectClocks) -> &mut Option<Refresh>) {
        *f(self) = Some(Refresh::new(time * self.duration_time_mult()));
    }

    fn duration_time_mult(&self) -> f64 {
        self.duration_time_mult.unwrap_or(1.0)
    }
}

macros::serialize_via_state!(EffectClocks => Option<f64> as |c| c.duration_time_mult);
macros::deserialize_via_state!(EffectClocks => Option<f64> as EffectClocks::from_duration_time_mult);

#[allow(unused)]
mod prelude {
    pub(crate) use super::*;
    pub(crate) use crate::calc;
}
