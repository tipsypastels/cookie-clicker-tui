use super::util::modal::*;
use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph, Wrap},
};

pub fn debug(app: &mut App, area: Rect, buf: &mut Buffer) {
    let Some(message) = app.debug_message() else {
        return;
    };

    let area = Modal::Small.split(area);

    Clear.render(area, buf);
    Paragraph::new(Text::raw(message))
        .wrap(Wrap { trim: false })
        .block(Block::new().style(Style::new().bg(Color::DarkGray)))
        .render(area, buf);
}
