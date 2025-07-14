mod buildings;
mod cookies;
mod ticker;
mod upgrades;
mod util;

use crate::app::App;
use ratatui::prelude::*;

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::White)
    .fg(Color::Black)
    .add_modifier(Modifier::BOLD);

pub fn ui(app: &mut App, f: &mut Frame) {
    let area = f.area();
    let buf = f.buffer_mut();

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .spacing(1)
        .split(area);

    main_column(app, cols[0], buf);
    buildings::buildings(app, cols[1], buf);
    upgrades::upgrades(app, cols[2], buf);
}

fn main_column(app: &mut App, area: Rect, buf: &mut Buffer) {
    // TODO: Why the extra spacing?
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Length(3)])
        .split(area);

    cookies::cookies(app, rows[0], buf);
    ticker::ticker(app, rows[1], buf);
}
