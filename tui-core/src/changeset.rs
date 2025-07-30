#[derive(Default, Debug, PartialEq)]
#[non_exhaustive]
pub struct Changeset {
    pub cps: bool,
    pub buildings_count: bool,
    pub grandmas_count: bool,
    pub available_upgrades: bool,
    pub research_completed: bool,
    pub sugar_lumps_unlocked: bool,
}
