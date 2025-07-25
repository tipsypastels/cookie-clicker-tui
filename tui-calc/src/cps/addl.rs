pub struct Cps<GrandmapocalypseMults> {
    pub grandmapocalypse_mults: GrandmapocalypseMults,
}

impl<GrandmapocalypseMults> Cps<GrandmapocalypseMults>
where
    GrandmapocalypseMults: Iterator<Item = f64>,
{
    pub fn calc(self, base: f64) -> f64 {
        let mut cps = base;

        // TODO: Add milk once you've ported it to the same system as vanilla.

        // Each grandmapocalypse mult is multiplied independantly.
        // These are expected to be floats of the form 1.04, for a 4% increase.
        for grandmapocalypse_mult in self.grandmapocalypse_mults {
            cps *= grandmapocalypse_mult;
        }

        cps
    }
}
