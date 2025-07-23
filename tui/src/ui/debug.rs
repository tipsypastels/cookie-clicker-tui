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
        AppDebugView::Cps => format!("{:?}", core.cps()),
        AppDebugView::Buildings => format!("{:?}", core.debug_buildings()),
        AppDebugView::AvailableUpgrades => format!("{:?}", core.debug_available_upgrades()),
        AppDebugView::OwnedUpgrades => format!("{:?}", core.owned_upgrades()),
        AppDebugView::Achievements => format!("{:?}", core.debug_achievements()),
        AppDebugView::Milk => format!("{:?}", core.milk()),
        AppDebugView::SugarLumps => format!("{:?}", core.sugar_lumps()),
        AppDebugView::Research => format!("{:?}", core.research()),
        AppDebugView::Ticker => format!("{:?}", core.debug_ticker()),
        AppDebugView::List => format!("{:?}", app.list.debug(core)),
        AppDebugView::Keypress => {
            if let Some(key) = app.debug.latest_key_event() {
                format!("{key:?}")
            } else {
                "".to_string()
            }
        }
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
