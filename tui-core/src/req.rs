use crate::{Achievement, Building, Computed, GrandmapocalypsePhase, State};

#[allow(unused)]
pub enum Req {
    Cookies(Cmp<f64>),
    CookiesAllTime(Cmp<f64>),
    CookiesAllTimeFromClicking(Cmp<f64>),
    BuildingCountMin(Building, u16),
    ResearchCompleted(Cmp<u8>),
    Achievement(Achievement),
    AchievementCount(Cmp<usize>),
    GrandmaJobUpgradeCount(Cmp<u16>),
    GrandmapocalypsePhase(GrandmapocalypsePhase),
    GrandmapocalypseAppeased(),
    GoldenCookieClicked(Cmp<usize>),
    GoldenCookieClickedAtMost1sAfterSpawn(),
    GoldenCookieClickedAtMost1sBeforeDespawn(),
    Custom(fn(&State) -> bool),
    Any(&'static [Req]),
    AnyBox(Box<[Req]>),
    All(&'static [Req]),
    AllBox(Box<[Req]>),
}

impl Req {
    pub fn check(&self, state: &State) -> bool {
        match self {
            Self::Cookies(c) => c.check(state.cookies.current()),
            Self::CookiesAllTime(c) => c.check(state.cookies.all_time()),
            Self::CookiesAllTimeFromClicking(c) => c.check(state.cookies.all_time_from_clicking()),
            Self::BuildingCountMin(b, c) => state.buildings.count(*b) >= *c,
            Self::ResearchCompleted(c) => c.check(state.research.completed()),
            Self::Achievement(a) => state.achievements.owned().contains(a),
            Self::AchievementCount(c) => c.check(state.achievements.owned().len()),
            Self::GrandmaJobUpgradeCount(c) => c.check(state.buildings.grandma_job_upgrade_count()),
            Self::GrandmapocalypsePhase(p) => state.grandmapocalypse.is_phase(*p),
            Self::GrandmapocalypseAppeased() => state.grandmapocalypse.is_appeased(),
            Self::GoldenCookieClicked(c) => c.check(state.golden_cookies.click_count()),
            Self::GoldenCookieClickedAtMost1sAfterSpawn() => {
                state.golden_cookies.clicked_one_at_most_1s_after_spawn()
            }
            Self::GoldenCookieClickedAtMost1sBeforeDespawn() => {
                state.golden_cookies.clicked_one_at_most_1s_before_despawn()
            }
            Self::Custom(f) => f(state),
            Self::Any(reqs) => reqs.iter().any(|r| r.check(state)),
            Self::AnyBox(reqs) => reqs.iter().any(|r| r.check(state)),
            Self::All(reqs) => reqs.iter().all(|r| r.check(state)),
            Self::AllBox(reqs) => reqs.iter().all(|r| r.check(state)),
        }
    }
}

#[allow(unused)]
pub enum LateReq {
    Req(Req),
    Cps(Cmp<f64>),
    Custom(fn(&State, &Computed) -> bool),
    Any(&'static [LateReq]),
    AnyBox(Box<[LateReq]>),
    All(&'static [LateReq]),
    AllBox(Box<[LateReq]>),
}

macro_rules! delegated_late_variants {
    ($($fn:ident($($arg:ident: $ty:ty),*);)*) => {
        $(#[allow(unused, non_snake_case)] pub const fn $fn($($arg: $ty),*) -> Self {
            Self::Req(Req::$fn($($arg),*))
        })*
    };
}

impl LateReq {
    delegated_late_variants! {
        Cookies(c: Cmp<f64>);
        CookiesAllTime(c: Cmp<f64>);
        CookiesAllTimeFromClicking(c: Cmp<f64>);
        BuildingCountMin(b: Building, c: u16);
        ResearchCompleted(c: Cmp<u8>);
        Achievement(a: Achievement);
        AchievementCount(c: Cmp<usize>);
        GrandmaJobUpgradeCount(c: Cmp<u16>);
        GrandmapocalypsePhase(p: GrandmapocalypsePhase);
        GrandmapocalypseAppeased();
        GoldenCookieClicked(c: Cmp<usize>);
        GoldenCookieClickedAtMost1sAfterSpawn();
        GoldenCookieClickedAtMost1sBeforeDespawn();
    }

    pub fn check(&self, state: &State, computed: &Computed) -> bool {
        match self {
            Self::Req(req) => req.check(state),
            Self::Cps(c) => c.check(computed.cps),
            Self::Custom(f) => f(state, computed),
            Self::Any(reqs) => reqs.iter().any(|r| r.check(state, computed)),
            Self::AnyBox(reqs) => reqs.iter().any(|r| r.check(state, computed)),
            Self::All(reqs) => reqs.iter().all(|r| r.check(state, computed)),
            Self::AllBox(reqs) => reqs.iter().all(|r| r.check(state, computed)),
        }
    }
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Cmp<T> {
    Above(T),
    AboveOrEq(T),
    Below(T),
    BelowOrEq(T),
    Range(T, T),
}

impl<T: PartialOrd> Cmp<T> {
    fn check(self, value: T) -> bool {
        match self {
            Self::Above(v) => value > v,
            Self::AboveOrEq(v) => value >= v,
            Self::Below(v) => value < v,
            Self::BelowOrEq(v) => value <= v,
            Self::Range(a, b) => (a..b).contains(&value),
        }
    }
}
