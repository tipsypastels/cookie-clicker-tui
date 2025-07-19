use super::UiApp;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn ticker(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some(text) = app.core.ticker() else {
        return Block::bordered().render(area, buf);
    };

    Paragraph::new(text)
        .centered()
        .block(Block::bordered())
        .render(area, buf);
}
