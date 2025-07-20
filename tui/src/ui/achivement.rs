use super::{UiApp, utils::num::PrintFloat};
use cookie_clicker_tui_core::AchivementReq;
use cookie_clicker_tui_utils::pluralized;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};

const SCREEN_PERCENT: (u16, u16) = (33, 15);

pub fn achivement(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some(achivement) = app.core.queued_achivement() else {
        return;
    };

    let title = Line::styled(
        format!(" Achivement Unlocked: {} ", achivement.name()),
        Modifier::BOLD,
    );

    let req = match achivement.req() {
        AchivementReq::CookiesBaked(n) => {
            format!(
                "• bake {} {}",
                n.print_float(0, 0),
                pluralized(n as _, "cookie", "cookies")
            )
        }
    };

    let area = split_area(area);
    let block = Block::bordered()
        .style(Style::new().bg(Color::DarkGray))
        .border_style(Style::new().black())
        .title(title);

    Clear.render(area, buf);
    Paragraph::new(req).block(block).render(area, buf);
}

fn split_area(area: Rect) -> Rect {
    let (percent_x, percent_y) = SCREEN_PERCENT;
    let vert = Layout::vertical([
        Constraint::Percentage(100 - percent_y),
        Constraint::Percentage(percent_y),
    ]);
    let horiz = Layout::horizontal([
        Constraint::Percentage(100 - percent_x),
        Constraint::Percentage(percent_x),
    ]);
    horiz.split(vert.split(area)[1])[1]
}
