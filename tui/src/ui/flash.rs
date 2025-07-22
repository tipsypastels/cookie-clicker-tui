use super::UiApp;
use crate::app::AppFlash;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};

const SCREEN_PERCENT: (u16, u16) = (33, 15);

pub fn flash(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some(flash) = app.iface.flash() else {
        return;
    };

    let text = match flash {
        AppFlash::CantAffordBuilding(building) => {
            format!("• you can't afford a {}", building.name_lower())
        }
        AppFlash::CantAffordUpgrade(index) => {
            let Some(upgrade) = &app.core.upgrades().get(index) else {
                return;
            };
            format!("• you can't afford {}", upgrade.label().to_lowercase())
        }
        AppFlash::CantSellUnownedBuilding(building) => {
            format!("• you don't have a {} to sell", building.name_lower())
        }
    };

    let title = flash.title();
    let style = flash.style();

    let area = split_area(area);
    let block = Block::bordered()
        .style(style)
        .border_style(Style::new().black())
        .title(title);

    Clear.render(area, buf);
    Paragraph::new(text).block(block).render(area, buf);
}

fn split_area(area: Rect) -> Rect {
    let (percent_x, percent_y) = SCREEN_PERCENT;
    let vert = Layout::vertical([
        Constraint::Percentage(percent_y),
        Constraint::Percentage(100 - percent_y),
    ]);
    let horiz = Layout::horizontal([
        Constraint::Percentage(percent_x),
        Constraint::Percentage(100 - percent_x),
    ]);
    horiz.split(vert.split(area)[0])[0]
}
