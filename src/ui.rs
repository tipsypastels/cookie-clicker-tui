use crate::{
    app::{App, ListStatePane},
    num::AsBigCountFmt,
};
use ratatui::{
    prelude::*,
    widgets::{Block, List, Padding, Paragraph},
};

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

    cookies_and_ticker_col(app, cols[0], buf);
    buildings_block(app, cols[1], buf);
    upgrades_block(app, cols[2], buf);
}

fn cookies_and_ticker_col(app: &mut App, area: Rect, buf: &mut Buffer) {
    // TODO: Why the extra spacing?
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Length(3)])
        .split(area);

    cookies_block(app, rows[0], buf);
    ticker_block(app, rows[1], buf);
}

fn cookies_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    let logo_line = |s: &'static str| {
        if app.just_pressed_cookie() {
            Line::styled(s, Color::Green)
        } else {
            Line::raw(s)
        }
    };

    Paragraph::new(Text::from(vec![
        Line::styled(
            format!("{}", app.cookies().as_big_count_fmt()),
            Style {
                fg: app.error_insufficient_cookies().then_some(Color::Red),
                add_modifier: Modifier::BOLD,
                ..Default::default()
            },
        ),
        Line::styled(
            format!("(per second: {:.1})", app.buildings().cps()),
            Modifier::ITALIC,
        ),
        Line::default(),
        logo_line("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡴⠚⣉⡙⠲⠦⠤⠤⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀"),
        logo_line("⠀⠀⠀⠀⠀⠀⢀⣴⠛⠉⠉⠀⣾⣷⣿⡆⠀⠀⠀⠐⠛⠿⢟⡲⢦⡀⠀⠀⠀⠀"),
        logo_line("⠀⠀⠀⠀⣠⢞⣭⠎⠀⠀⠀⠀⠘⠛⠛⠀⠀⢀⡀⠀⠀⠀⠀⠈⠓⠿⣄⠀⠀⠀"),
        logo_line("⠀⠀⠀⡜⣱⠋⠀⠀⣠⣤⢄⠀⠀⠀⠀⠀⠀⣿⡟⣆⠀⠀⠀⠀⠀⠀⠻⢷⡄⠀"),
        logo_line("⠀⢀⣜⠜⠁⠀⠀⠀⢿⣿⣷⣵⠀⠀⠀⠀⠀⠿⠿⠿⠀⠀⣴⣶⣦⡀⠀⠰⣹⡆"),
        logo_line("⢀⡞⠆⠀⣀⡀⠀⠀⠘⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣶⠇⠀⢠⢻⡇"),
        logo_line("⢸⠃⠘⣾⣏⡇⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⣠⣤⣤⡉⠁⠀⠀⠈⠫⣧"),
        logo_line("⡸⡄⠀⠘⠟⠀⠀⠀⠀⠀⠀⣰⣿⣟⢧⠀⠀⠀⠀⠰⡿⣿⣿⢿⠀⠀⣰⣷⢡⢸"),
        logo_line("⣿⡇⠀⠀⠀⣰⣿⡻⡆⠀⠀⠻⣿⣿⣟⠀⠀⠀⠀⠀⠉⠉⠉⠀⠀⠘⢿⡿⣸⡞"),
        logo_line("⠹⣽⣤⣤⣤⣹⣿⡿⠇⠀⠀⠀⠀⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡔⣽⠀"),
        logo_line("⠀⠙⢻⡙⠟⣹⠟⢷⣶⣄⢀⣴⣶⣄⠀⠀⠀⠀⠀⢀⣤⡦⣄⠀⠀⢠⣾⢸⠏⠀"),
        logo_line("⠀⠀⠘⠀⠀⠀⠀⠀⠈⢷⢼⣿⡿⡽⠀⠀⠀⠀⠀⠸⣿⣿⣾⠀⣼⡿⣣⠟⠀⠀"),
        logo_line("⠀⠀⠀⠀⠀⠀⠀⠀⢠⡾⣆⠑⠋⠀⢀⣀⠀⠀⠀⠀⠈⠈⢁⣴⢫⡿⠁⠀⠀⠀"),
        logo_line("⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⣧⣄⡄⠴⣿⣶⣿⢀⣤⠶⣞⣋⣩⣵⠏⠀⠀⠀⠀"),
        logo_line("⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⣿⢯⣭⣭⣯⣯⣥⡵⠿⠟⠛⠉⠉⠀⠀⠀⠀"),
    ]))
    .centered()
    .block(Block::bordered().title(Line::styled(" Cookies ", Modifier::BOLD).centered()))
    .render(area, buf);
}

fn ticker_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    if let Some(text) = app.ticker() {
        Paragraph::new(Text::from(text))
            .centered()
            .block(Block::bordered())
            .render(area, buf);
    } else {
        Block::bordered().render(area, buf);
    }
}

fn buildings_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    List::new(app.buildings().iter().map(|(building, count)| {
        format!(
            "{} {} ({} cookies)",
            count,
            building.name_pluralized(count as _),
            app.buildings().cost(building).as_big_count_fmt(),
        )
    }))
    .highlight_style(SELECTED_STYLE)
    .block(
        Block::bordered()
            .title(Line::styled(" Buildings ", Modifier::BOLD).centered())
            .padding(Padding::uniform(1)),
    )
    .render_maybe_stateful(area, buf, app.list_state_for_pane(ListStatePane::Buildings));
}

fn upgrades_block(app: &mut App, area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .title(Line::styled(" Upgrades ", Modifier::BOLD).centered())
        .render(area, buf);
}

trait WidgetExt: Widget + Sized {
    fn render_stateless(self, area: Rect, buf: &mut Buffer) {
        Self::render(self, area, buf);
    }
}

trait StatefulWidgetExt: StatefulWidget + Sized {
    fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Self::render(self, area, buf, state);
    }
}

trait MaybeStatefulWidgetExt: Widget + StatefulWidget + Sized {
    fn render_maybe_stateful(self, area: Rect, buf: &mut Buffer, state: Option<&mut Self::State>) {
        if let Some(state) = state {
            self.render_stateful(area, buf, state);
        } else {
            self.render_stateless(area, buf);
        }
    }
}

impl<W: Widget> WidgetExt for W {}
impl<W: StatefulWidget> StatefulWidgetExt for W {}
impl<W: Widget + StatefulWidget> MaybeStatefulWidgetExt for W {}
