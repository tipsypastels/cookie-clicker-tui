pub struct Cps<BuildingCpses> {
    pub building_cpses: BuildingCpses,
}

impl<BuildingCpses> Cps<BuildingCpses>
where
    BuildingCpses: Iterator<Item = f64>,
{
    pub fn calc(self) -> f64 {
        self.building_cpses.sum()
    }
}
