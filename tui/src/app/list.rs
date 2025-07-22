use cookie_clicker_tui_core::{Building, Core, Upgrade};
use std::fmt;
use tui_widget_list::ListState;

#[derive(Default)]
pub struct AppListState {
    state: ListState,
    pane: AppListPane,
}

impl AppListState {
    pub fn pointee<'a>(&self, core: &'a Core) -> Option<(usize, AppListPointee<'a>)> {
        let index = self.state.selected?;
        match self.pane {
            AppListPane::Buildings => {
                Some((index, AppListPointee::Building(Building::nth(index)?)))
            }
            AppListPane::Upgrades => {
                Some((index, AppListPointee::Upgrade(core.upgrades().get(index)?)))
            }
        }
    }

    pub fn debug(&self, core: &Core) -> impl fmt::Debug {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct AppListDebug<'list, 'core> {
            state: &'list ListState,
            pane: AppListPane,
            pointee: Option<(usize, AppListPointee<'core>)>,
        }
        AppListDebug {
            state: &self.state,
            pane: self.pane,
            pointee: self.pointee(core),
        }
    }

    pub(super) fn up(&mut self) {
        self.state.previous();
    }

    pub(super) fn down(&mut self) {
        self.state.next();
    }

    pub(super) fn left(&mut self, core: &Core) {
        self.lr(core, AppListPane::prev);
    }

    pub(super) fn right(&mut self, core: &Core) {
        self.lr(core, AppListPane::next)
    }

    fn lr(&mut self, core: &Core, change: fn(AppListPane) -> AppListPane) {
        let mut new_pane = change(self.pane);
        loop {
            if new_pane.available(core) {
                break;
            }
            new_pane = change(new_pane);
        }

        self.pane = new_pane;
        self.state.select(Some(0));
    }

    pub fn is_pane(&self, pane: AppListPane) -> bool {
        self.pane == pane
    }

    pub fn state_matching_mut(&mut self, pane: AppListPane) -> Option<&mut ListState> {
        (self.pane == pane).then_some(&mut self.state)
    }
}

#[derive(Debug)]
pub enum AppListPointee<'a> {
    Building(Building),
    Upgrade(&'a Upgrade),
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum AppListPane {
    #[default]
    Buildings,
    Upgrades,
}

impl AppListPane {
    fn available(self, core: &Core) -> bool {
        match self {
            Self::Buildings => true,
            Self::Upgrades => !core.upgrades().is_empty(),
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
