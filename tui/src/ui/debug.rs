use super::{UiApp, utils::modal::*};
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph, Wrap},
};

pub fn debug(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some(message) = app.countdown.debug_message() else {
        return;
    };

    let area = Modal::Small.split(area);

    Clear.render(area, buf);
    Paragraph::new(Text::raw(message))
        .wrap(Wrap { trim: false })
        .block(Block::new().style(Style::new().bg(Color::DarkGray)))
        .render(area, buf);
}
