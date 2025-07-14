use crate::app::App;
use ratatui::{prelude::*, widgets::Block};

pub fn upgrades(app: &mut App, area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .title(Line::styled(" Upgrades ", Modifier::BOLD))
        .render(area, buf)
}
