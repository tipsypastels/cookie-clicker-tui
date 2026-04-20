use super::prelude::*;

#[derive(Debug)]
pub struct Lucky;

impl EffectImpl for Lucky {
    fn effect(
        self,
        cps: &Cps,
        cookies: &mut Cookies,
        _clocks: &mut EffectClocks,
        _changeset: &mut Changeset,
    ) {
        let gain = calc::golden_cookie_lucky_gain(cps, cookies);
        cookies.gain_bulk(gain);
    }
}
