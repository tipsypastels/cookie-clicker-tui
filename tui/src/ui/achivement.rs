use super::UiApp;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};

const SCREEN_PERCENT: (u16, u16) = (40, 20);

pub fn achivement(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some(achivement) = app.core.queued_achivement() else {
        return;
    };

    let area = split_area(area);
    let block = Block::bordered()
        .style(Style::new().bg(Color::DarkGray))
        .border_style(Style::new().black())
        .title(Line::styled(" Achivement Unlocked ", Modifier::BOLD));

    Clear.render(area, buf);
    Paragraph::new(achivement.name())
        .block(block)
        .render(area, buf);
}

fn split_area(area: Rect) -> Rect {
    let (percent_x, percent_y) = SCREEN_PERCENT;
    let vert = Layout::vertical([
        Constraint::Percentage(100 - percent_y),
        Constraint::Percentage(percent_y),
    ]);
    let horiz = Layout::horizontal([
        Constraint::Percentage(percent_x),
        Constraint::Percentage(100 - percent_x),
    ]);
    horiz.split(vert.split(area)[1])[1]
}
