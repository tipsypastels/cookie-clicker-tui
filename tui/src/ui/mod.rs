mod achivement;
mod buildings;
mod cookies;
mod modal;
mod ticker;
mod upgrades;
mod utils;

use crate::app::{AppCountdownState, AppListState, AppModalState};
use cookie_clicker_tui_core::Core;
use ratatui::prelude::*;

#[derive(Debug)]
pub struct UiApp<'a> {
    pub core: &'a Core,
    pub list: &'a mut AppListState,
    pub countdown: &'a AppCountdownState,
    pub modal: &'a AppModalState,
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

    achivement::achivement(app, area, buf);
    modal::modal(app, area, buf);
}

fn left_col(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([Constraint::Percentage(100), Constraint::Length(3)]).split(area);

    cookies::cookies(app, rows[0], buf);
    ticker::ticker(app, rows[1], buf);
}
