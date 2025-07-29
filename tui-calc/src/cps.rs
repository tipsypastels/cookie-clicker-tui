pub mod addl;
pub mod base;
pub mod building;
pub mod kittens;

#[non_exhaustive]
pub struct Cps {
    pub base: f64,
    pub total: f64,
    pub wrinkled: f64,
}

impl Cps {
    pub fn new<BuildingCpses, GrandmapocalypseMults>(
        base: base::Cps<BuildingCpses>,
        addl: addl::Cps<GrandmapocalypseMults>,
    ) -> Self
    where
        BuildingCpses: Iterator<Item = f64>,
        GrandmapocalypseMults: Iterator<Item = f64>,
    {
        let base = base.calc();
        let (total, wrinkled) = addl.calc(base);

        Self {
            base,
            total,
            wrinkled,
        }
    }
}
