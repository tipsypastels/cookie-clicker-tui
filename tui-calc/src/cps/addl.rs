pub struct Cps<GrandmapocalypseMults> {
    pub grandmapocalypse_mults: GrandmapocalypseMults,
    pub kitten_mult: f64,
    pub wrinkler_count: usize,
    pub has_elder_covenant: bool,
}

impl<GrandmapocalypseMults> Cps<GrandmapocalypseMults>
where
    GrandmapocalypseMults: Iterator<Item = f64>,
{
    pub fn calc(self, base: f64) -> Calced {
        let mut cps = base;
        let mut wrinkled = 0.0;
        let mut debuff_ratio = 0.0;

        // Each grandmapocalypse mult is multiplied independantly.
        // These are expected to be floats of the form 1.04, for a 4% increase.
        for grandmapocalypse_mult in self.grandmapocalypse_mults {
            cps *= grandmapocalypse_mult;
        }

        // The kitten mult is pre-calculated.
        cps *= self.kitten_mult;

        if self.wrinkler_count > 0 {
            let mult = 0.05 * self.wrinkler_count as f64;

            wrinkled = cps * mult;
            cps -= wrinkled;
            debuff_ratio += mult;
        }

        if self.has_elder_covenant {
            cps *= 0.95;
        }

        Calced {
            cps,
            wrinkled,
            debuff_ratio,
        }
    }
}

#[non_exhaustive]
pub struct Calced {
    pub cps: f64,
    pub wrinkled: f64,
    pub debuff_ratio: f64,
}
