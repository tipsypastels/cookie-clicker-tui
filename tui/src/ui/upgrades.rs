use super::UiApp;
use ratatui::{prelude::*, widgets::Block};

pub fn upgrades(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .title(Line::styled(" Upgrades ", Modifier::BOLD))
        .render(area, buf)
}
