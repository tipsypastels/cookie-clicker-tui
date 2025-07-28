mod achievement;
mod building;
mod calc;
mod click;
mod cookies;
mod cost;
mod golden_cookie;
mod grandmapocalypse;
mod macros;
mod milk;
mod news;
mod req;
mod research;
mod spawner;
mod sugar_lumps;
mod thousand_fingers;
mod upgrade;

pub use self::{
    achievement::{Achievement, AchievementReq},
    building::{Building, BuildingInfo},
    cost::{Cost, CostDyn, CostResolved},
    golden_cookie::{GoldenCookie, GoldenCookies},
    grandmapocalypse::{Grandmapocalypse, GrandmapocalypsePhase},
    milk::{Milk, MilkFlavor},
    news::NewsEntry,
    research::Research,
    sugar_lumps::SugarLumps,
    upgrade::{
        Upgrade, UpgradeEffectInfo, UpgradeInfoEffectResearch, UpgradeInfoEffectResearchWarning,
    },
};

use self::{
    achievement::Achievements,
    building::Buildings,
    click::Click,
    cookies::Cookies,
    thousand_fingers::ThousandFingers,
    upgrade::{AvailableUpgrades, OwnedUpgrades},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fmt};

pub struct Core {
    state: State,
    computed: Computed,
    everything_free: bool,
}

impl Core {
    pub fn new() -> Self {
        Self::from_state(State::new())
    }

    fn from_state(state: State) -> Self {
        let computed = Computed::new(&state);
        let everything_free = false;

        Self {
            state,
            computed,
            everything_free,
        }
    }

    pub fn cookies(&self) -> f64 {
        self.state.cookies.current()
    }

    pub fn cookies_all_time(&self) -> f64 {
        self.state.cookies.all_time()
    }

    pub fn cookies_all_time_from_clicking(&self) -> f64 {
        self.state.cookies.all_time_from_clicking()
    }

    pub fn cps(&self) -> f64 {
        self.computed.cps
    }

    pub fn milk(&self) -> &Milk {
        &self.state.milk
    }

    pub fn sugar_lumps(&self) -> &SugarLumps {
        &self.state.sugar_lumps
    }

    pub fn building_infos(&self) -> impl Iterator<Item = BuildingInfo> {
        self.state.buildings.infos()
    }

    pub fn building_info(&self, building: Building) -> BuildingInfo {
        self.state.buildings.info(building)
    }

    pub fn building_info_nth(&self, index: usize) -> BuildingInfo {
        self.state.buildings.info_nth(index)
    }

    pub fn owned_upgrades(&self) -> &BTreeSet<Upgrade> {
        self.state.owned_upgrades.as_set()
    }

    pub fn available_upgrades(&self) -> &[Upgrade] {
        &self.computed.available_upgrades
    }

    pub fn owned_achievements(&self) -> &BTreeSet<Achievement> {
        self.state.achievements.owned()
    }

    pub fn queued_achievement(&self) -> Option<Achievement> {
        self.state.achievements.queued()
    }

    pub fn research(&self) -> &Research {
        &self.state.research
    }

    pub fn grandmapocalypse(&self) -> &Grandmapocalypse {
        &self.state.grandmapocalypse
    }

    pub fn golden_cookies(&self) -> &GoldenCookies {
        &self.state.golden_cookies
    }

    pub fn random_news_entry(&self) -> Option<NewsEntry> {
        self::news::get_entry(&self.state)
    }

    pub fn affordable(&self, cost: Cost) -> bool {
        self.everything_free || cost.affordable(&self.state, self.computed.cps)
    }

    pub fn affordable_resolved(&self, cost: CostResolved) -> bool {
        self.everything_free || cost.affordable(&self.state)
    }

    pub fn resolve_cost(&self, cost: Cost) -> CostResolved {
        cost.resolve(&self.state, self.computed.cps)
    }

    pub fn click_cookie(&mut self) {
        self.state
            .cookies
            .gain_from_clicking(self.state.click.cpc());
    }

    pub fn click_golden_cookie(&mut self, ch: char) -> bool {
        self.state.golden_cookies.click(ch)
    }

    pub fn give_building(&mut self, building: Building) {
        self.state.buildings.modify_count(building, |c| *c += 1);
        self.computed.recalc_cps(&self.state);
        self.computed.recalc_available_upgrades(&self.state);
    }

    pub fn take_building(&mut self, building: Building) {
        self.state.buildings.modify_count(building, |c| *c -= 1);

        self.computed.recalc_cps(&self.state);
        // TODO: Grandmapocalypse adjusting to grandma changes
        // happens too late for the upgrades to change eagerly,
        // since it's called in tick, and therefore must wait
        // for upgrades to refresh instead.
        //
        // Consider using an event queue that's only handled
        // in tick instead of updating immediately.
        // This would also require drawing frames after
        // tick/events, maybe.
        self.computed.recalc_available_upgrades(&self.state);
    }

    pub fn buy_building(&mut self, building: Building) -> bool {
        let cost = self.resolve_cost(self.building_info(building).cost());

        if !self.affordable_resolved(cost) {
            return false;
        }

        if !self.everything_free {
            match cost {
                CostResolved::Cookies(cookies) => {
                    self.state.cookies.lose(cookies);
                }
            }
        }

        self.give_building(building);
        true
    }

    pub fn sell_building(&mut self, building: Building) -> bool {
        let info = self.building_info(building);

        if info.count() == 0 {
            return false;
        };

        if !self.everything_free {
            match self.resolve_cost(info.sell_cost()) {
                CostResolved::Cookies(cookies) => {
                    self.state.cookies.gain(cookies);
                }
            }
        }

        // TODO: Consider implementing this into `modify_count`
        // unless there are other ways to lose a grandma.
        if building.is_grandma() {
            self.state.buildings.set_grandma_been_sold(true);
        }

        self.take_building(building);
        true
    }

    pub fn buy_upgrade(&mut self, upgrade: Upgrade) -> bool {
        if !self.computed.available_upgrades.contains(&upgrade) {
            return false;
        };

        if self.state.owned_upgrades.has(upgrade) {
            return false;
        }

        let cost = self.resolve_cost(upgrade.cost());

        if !self.affordable_resolved(cost) {
            return false;
        }

        if !self.everything_free {
            match cost {
                CostResolved::Cookies(cookies) => {
                    self.state.cookies.lose(cookies);
                }
            }
        }

        if upgrade.should_add_to_owned() {
            self.state.owned_upgrades.add(upgrade);
        }

        upgrade.buy(&mut self.state);

        self.computed.recalc_cps(&self.state);
        self.computed.recalc_available_upgrades(&self.state);

        true
    }

    pub fn cheat_make_everything_free(&mut self) {
        self.everything_free = true;
    }

    pub fn cheat_spawn_golden_cookies_fast(&mut self) {
        self.state.golden_cookies.modify_spawning(|min, max| {
            *min = 5.0;
            *max = 10.0;
        });
    }

    pub fn tick(&mut self) {
        self.state.tick(&self.computed);
        self.computed.tick(&self.state);
    }

    pub fn debug_cookies(&self) -> impl fmt::Debug {
        &self.state.cookies
    }

    pub fn debug_buildings(&self) -> impl fmt::Debug {
        &self.state.buildings
    }

    pub fn debug_buildings_flags(&self) -> impl fmt::Debug {
        self.state.buildings.debug_flags()
    }

    pub fn debug_click(&self) -> impl fmt::Debug {
        &self.state.click
    }

    pub fn debug_available_upgrades(&self) -> impl fmt::Debug {
        &self.computed.available_upgrades
    }

    pub fn debug_achievements(&self) -> impl fmt::Debug {
        &self.state.achievements
    }

    pub fn debug_thousand_fingers(&self) -> impl fmt::Debug {
        &self.state.thousand_fingers
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

macros::serialize_via_state!(Core => State as |c| c.state);
macros::deserialize_via_state!(Core => State as Core::from_state);

#[derive(Serialize, Deserialize)]
struct State {
    cookies: Cookies,
    buildings: Buildings,
    click: Click,
    milk: Milk,
    achievements: Achievements,
    owned_upgrades: OwnedUpgrades,
    thousand_fingers: ThousandFingers,
    sugar_lumps: SugarLumps,
    research: Research,
    grandmapocalypse: Grandmapocalypse,
    golden_cookies: GoldenCookies,
}

impl State {
    fn new() -> Self {
        Self {
            cookies: Cookies::new(),
            buildings: Buildings::new(),
            click: Click::new(),
            milk: Milk::new(),
            achievements: Achievements::new(),
            owned_upgrades: OwnedUpgrades::new(),
            thousand_fingers: ThousandFingers::new(),
            sugar_lumps: SugarLumps::new(),
            research: Research::new(),
            grandmapocalypse: Grandmapocalypse::new(),
            golden_cookies: GoldenCookies::new(),
        }
    }

    fn tick(&mut self, computed: &Computed) {
        if self.buildings.total_count_just_changed() {
            self.click.set_non_cursor_buildings_count(
                self.buildings.total_count() - self.buildings.count(Building::Cursor),
            );
        }

        self.cookies.tick(computed.cps);
        self.buildings.tick();
        self.milk.tick(self.achievements.owned().len() as _);
        self.research.tick();
        self.grandmapocalypse
            .tick(self.buildings.count(Building::Grandma));
        self.golden_cookies.tick();

        achievement::tick(self, computed);
        sugar_lumps::tick(self);
    }
}

struct Computed {
    cps: f64,
    available_upgrades: AvailableUpgrades,
}

impl Computed {
    fn new(state: &State) -> Self {
        let cps = self::calc::cps(state);
        let available_upgrades = AvailableUpgrades::new(state, cps);

        Self {
            cps,
            available_upgrades,
        }
    }

    fn tick(&mut self, state: &State) {
        self.available_upgrades.tick(state, self.cps);

        if state.research.just_completed() {
            self.recalc_available_upgrades(state);
        }
    }

    fn recalc_cps(&mut self, state: &State) {
        self.cps = self::calc::cps(state);
    }

    fn recalc_available_upgrades(&mut self, state: &State) {
        self.available_upgrades = AvailableUpgrades::new(state, self.cps);
    }
}
