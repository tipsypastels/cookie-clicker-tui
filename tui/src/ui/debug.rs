use super::UiApp;
use crate::app::AppDebugView;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph, Wrap},
};

pub fn debug(app: &mut UiApp, view: AppDebugView, area: Rect, buf: &mut Buffer) {
    let core = app.core;

    let title = format!(" Debug - {} ", view.name());
    let text = match view {
        AppDebugView::Cookies => format!("{:?}", core.debug_cookies()),
        AppDebugView::Buildings => format!("{:?}", core.debug_buildings()),
        AppDebugView::Upgrades => format!("{:?}", core.debug_upgrades()),
    };

    let block = Block::bordered()
        .title(Line::styled(title, Modifier::BOLD).centered())
        .title_bottom(Line::styled(" More </> Close <Esc> ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .block(block)
        .render(area, buf);
}
