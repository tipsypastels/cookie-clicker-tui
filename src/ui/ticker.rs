use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn ticker(app: &mut App, area: Rect, buf: &mut Buffer) {
    let Some(text) = app.ticker() else {
        return Block::bordered().render(area, buf);
    };

    Paragraph::new(text)
        .centered()
        .block(Block::bordered())
        .render(area, buf);
}
