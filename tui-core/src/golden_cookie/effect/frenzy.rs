use super::prelude::*;

const DURATION_SECS: f64 = 77.0;

#[derive(Debug)]
pub struct Frenzy;

impl EffectImpl for Frenzy {
    fn effect(
        self,
        _cps: &Cps,
        _cookies: &mut Cookies,
        clocks: &mut EffectClocks,
        changeset: &mut Changeset,
    ) {
        clocks.start(DURATION_SECS, |clocks| &mut clocks.frenzy);
        changeset.cps = true;
    }
}
