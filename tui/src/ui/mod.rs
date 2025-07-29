mod achievement;
mod buildings;
mod cookies;
mod debug;
mod flash;
mod golden_cookies;
mod modal;
mod news;
mod upgrades;
mod utils;

use crate::{
    app::{
        AppBakery, AppDebugState, AppInterfaceState, AppListState, AppModalState, AppNewsState,
        AppTickState,
    },
    save::Save,
};
use cookie_clicker_tui_core::Core;
use ratatui::prelude::*;

pub struct UiApp<'a> {
    pub save: &'a Save,
    pub core: &'a Core,
    pub tick: &'a AppTickState,
    pub list: &'a mut AppListState,
    pub iface: &'a AppInterfaceState,
    pub modal: &'a mut AppModalState,
    pub news: &'a mut AppNewsState,
    pub debug: &'a AppDebugState,
    pub bakery: &'a AppBakery,
}

pub fn ui(app: &mut UiApp, frame: &mut Frame) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let cols = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .spacing(1)
    .split(area);

    left_col(app, cols[0], buf);

    buildings::buildings(app, cols[1], buf);
    upgrades::upgrades(app, cols[2], buf);

    flash::flash(app, area, buf);
    achievement::achievement(app, area, buf);
    modal::modal(app, area, buf);

    golden_cookies::golden_cookies(app, area, buf);
}

fn left_col(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([Constraint::Percentage(100), Constraint::Length(3)]).split(area);

    if let Some(view) = app.debug.view() {
        debug::debug(app, view, rows[0], buf);
    } else {
        cookies::cookies(app, rows[0], buf);
    }

    news::news(app, rows[1], buf);
}
