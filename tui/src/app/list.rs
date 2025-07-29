use cookie_clicker_tui_core::{Building, Core, Upgrade};
use std::{fmt, num::NonZero};
use tui_widget_list::ListState;

#[derive(Default)]
pub struct AppListState {
    buildings: ListState,
    upgrades: ListState,
    pane: AppListPane,
}

impl AppListState {
    pub fn pointee(&self, core: &Core) -> Option<AppListPointee> {
        let index = self.state(self.pane).selected?;
        match self.pane {
            AppListPane::Buildings => Some(AppListPointee::Building(Building::nth(index)?)),
            AppListPane::Upgrades => Some(AppListPointee::Upgrade(
                *core.available_upgrades().get(index)?,
            )),
        }
    }

    pub fn debug(&self, core: &Core) -> impl fmt::Debug {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct AppListDebug<'a> {
            buildings: &'a ListState,
            upgrades: &'a ListState,
            pane: AppListPane,
            pointee: Option<AppListPointee>,
        }
        AppListDebug {
            buildings: &self.buildings,
            upgrades: &self.upgrades,
            pane: self.pane,
            pointee: self.pointee(core),
        }
    }

    pub fn is_pane_highlighted(&self, pane: AppListPane) -> bool {
        self.pane == pane && self.state(pane).selected.is_some()
    }

    pub fn get_for_render(&mut self, pane: AppListPane, core: &Core) -> (bool, &mut ListState) {
        let selected = self.pane == pane;
        let state = self.state_mut(pane);

        match (pane.test(core), &mut state.selected) {
            (PaneTestRes::AvailableListLen(len), Some(n)) if *n >= len.get() => {
                *n = len.get() - 1;
            }
            _ => {}
        }

        (selected, state)
    }

    pub(super) fn up(&mut self) {
        self.state_mut(self.pane).previous();
    }

    pub(super) fn down(&mut self) {
        self.state_mut(self.pane).next();
    }

    pub(super) fn left(&mut self, core: &Core) {
        self.lr(core, AppListPane::prev);
    }

    pub(super) fn right(&mut self, core: &Core) {
        self.lr(core, AppListPane::next)
    }

    fn lr(&mut self, core: &Core, change: fn(AppListPane) -> AppListPane) {
        let mut new_pane = change(self.pane);
        let mut test_res = new_pane.test(core);

        loop {
            if test_res.is_available() {
                break;
            }
            new_pane = change(new_pane);
            test_res = new_pane.test(core);
        }

        self.pane = new_pane;

        match (test_res, &mut self.state_mut(self.pane).selected) {
            (_, selected @ None) => {
                *selected = Some(0);
            }
            (PaneTestRes::AvailableListLen(len), Some(n)) if *n >= len.get() => {
                *n = len.get() - 1;
            }
            _ => {}
        }
    }

    fn state(&self, pane: AppListPane) -> &ListState {
        match pane {
            AppListPane::Buildings => &self.buildings,
            AppListPane::Upgrades => &self.upgrades,
        }
    }

    fn state_mut(&mut self, pane: AppListPane) -> &mut ListState {
        match pane {
            AppListPane::Buildings => &mut self.buildings,
            AppListPane::Upgrades => &mut self.upgrades,
        }
    }
}

#[derive(Debug)]
pub enum AppListPointee {
    Building(Building),
    Upgrade(Upgrade),
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum AppListPane {
    #[default]
    Buildings,
    Upgrades,
}

impl AppListPane {
    fn test(self, core: &Core) -> PaneTestRes {
        match self {
            Self::Buildings => PaneTestRes::Available,
            Self::Upgrades => PaneTestRes::available_list_len(core.available_upgrades().len()),
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::Buildings => Self::Upgrades,
            Self::Upgrades => Self::Buildings,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Buildings => Self::Upgrades,
            Self::Upgrades => Self::Buildings,
        }
    }
}

enum PaneTestRes {
    Unavailable,
    Available,
    AvailableListLen(NonZero<usize>),
}

impl PaneTestRes {
    fn available_list_len(n: usize) -> Self {
        NonZero::new(n)
            .map(Self::AvailableListLen)
            .unwrap_or(Self::Unavailable)
    }

    fn is_available(&self) -> bool {
        matches!(self, Self::Available | Self::AvailableListLen(_))
    }
}
