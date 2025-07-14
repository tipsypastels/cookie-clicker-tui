use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, List, Paragraph},
};

pub fn ui(app: &mut App, f: &mut Frame) {
    let area = f.area();
    let buf = f.buffer_mut();

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(1)
        .split(area);

    cookies_and_ticker_col(app, cols[0], buf);
    buildings_block(app, cols[1], buf);
}

fn cookies_and_ticker_col(app: &mut App, area: Rect, buf: &mut Buffer) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(area);

    cookies_block(app, rows[0], buf);
    ticker_block(app, rows[1], buf);
}

fn cookies_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    Paragraph::new(Text::from(vec![
        Line::styled(format!("{}", app.state.cookies.value()), Modifier::BOLD),
        Line::styled(
            format!("(per second: {:.1})", app.state.buildings.cps()),
            Modifier::ITALIC,
        ),
        Line::default(),
        Line::raw("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡴⠚⣉⡙⠲⠦⠤⠤⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀"),
        Line::raw("⠀⠀⠀⠀⠀⠀⢀⣴⠛⠉⠉⠀⣾⣷⣿⡆⠀⠀⠀⠐⠛⠿⢟⡲⢦⡀⠀⠀⠀⠀"),
        Line::raw("⠀⠀⠀⠀⣠⢞⣭⠎⠀⠀⠀⠀⠘⠛⠛⠀⠀⢀⡀⠀⠀⠀⠀⠈⠓⠿⣄⠀⠀⠀"),
        Line::raw("⠀⠀⠀⡜⣱⠋⠀⠀⣠⣤⢄⠀⠀⠀⠀⠀⠀⣿⡟⣆⠀⠀⠀⠀⠀⠀⠻⢷⡄⠀"),
        Line::raw("⠀⢀⣜⠜⠁⠀⠀⠀⢿⣿⣷⣵⠀⠀⠀⠀⠀⠿⠿⠿⠀⠀⣴⣶⣦⡀⠀⠰⣹⡆"),
        Line::raw("⢀⡞⠆⠀⣀⡀⠀⠀⠘⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣶⠇⠀⢠⢻⡇"),
        Line::raw("⢸⠃⠘⣾⣏⡇⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⣠⣤⣤⡉⠁⠀⠀⠈⠫⣧"),
        Line::raw("⡸⡄⠀⠘⠟⠀⠀⠀⠀⠀⠀⣰⣿⣟⢧⠀⠀⠀⠀⠰⡿⣿⣿⢿⠀⠀⣰⣷⢡⢸"),
        Line::raw("⣿⡇⠀⠀⠀⣰⣿⡻⡆⠀⠀⠻⣿⣿⣟⠀⠀⠀⠀⠀⠉⠉⠉⠀⠀⠘⢿⡿⣸⡞"),
        Line::raw("⠹⣽⣤⣤⣤⣹⣿⡿⠇⠀⠀⠀⠀⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡔⣽⠀"),
        Line::raw("⠀⠙⢻⡙⠟⣹⠟⢷⣶⣄⢀⣴⣶⣄⠀⠀⠀⠀⠀⢀⣤⡦⣄⠀⠀⢠⣾⢸⠏⠀"),
        Line::raw("⠀⠀⠘⠀⠀⠀⠀⠀⠈⢷⢼⣿⡿⡽⠀⠀⠀⠀⠀⠸⣿⣿⣾⠀⣼⡿⣣⠟⠀⠀"),
        Line::raw("⠀⠀⠀⠀⠀⠀⠀⠀⢠⡾⣆⠑⠋⠀⢀⣀⠀⠀⠀⠀⠈⠈⢁⣴⢫⡿⠁⠀⠀⠀"),
        Line::raw("⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⣧⣄⡄⠴⣿⣶⣿⢀⣤⠶⣞⣋⣩⣵⠏⠀⠀⠀⠀"),
        Line::raw("⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⣿⢯⣭⣭⣯⣯⣥⡵⠿⠟⠛⠉⠉⠀⠀⠀⠀"),
    ]))
    .centered()
    .block(Block::bordered().title(Line::styled(" Cookies ", Modifier::BOLD).centered()))
    .render(area, buf);
}

fn ticker_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    if let Some(text) = app.ticker.text() {
        Paragraph::new(Text::from(text))
            .centered()
            .block(Block::bordered())
            .render(area, buf);
    } else {
        Block::bordered().render(area, buf);
    }
}

fn buildings_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    List::new(
        app.state
            .buildings
            .iter()
            .map(|(building, count)| format!("{} ({})", building.name(), count)),
    )
    .highlight_symbol(">")
    .block(Block::bordered().title(Line::styled(" Buildings ", Modifier::BOLD).centered()))
    .render_stateful(area, buf, &mut app.building_list_state);
}

// hack to make it possible to call the method
// without needing to disambiguate with `Widget::render`
trait StatefulWidgetExt: StatefulWidget + Sized {
    fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Self::render(self, area, buf, state);
    }
}

impl<W: StatefulWidget> StatefulWidgetExt for W {}
