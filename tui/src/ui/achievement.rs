use super::{UiApp, utils::num::PrintFloat};
use cookie_clicker_tui_core::{AchievementReq, Building};
use cookie_clicker_tui_utils::str::pluralized;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};

const SCREEN_PERCENT: (u16, u16) = (33, 15);

pub fn achievement(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some(achievement) = app.core.queued_achievement() else {
        return;
    };

    let title = Line::styled(
        format!(" Achievement Unlocked: {} ", achievement.name()),
        Modifier::BOLD,
    );

    let req = match achievement.req() {
        AchievementReq::CookiesBaked(n) => {
            format!(
                "• bake {} {}",
                n.print_float(0, 0),
                pluralized(n as _, "cookie", "cookies")
            )
        }
        AchievementReq::Cps(n) => {
            format!(
                "• bake {} {} per second",
                n.print_float(0, 0),
                pluralized(n as _, "cookie", "cookies")
            )
        }
        AchievementReq::GrandmaJobCount(n) => {
            format!(
                "• have {n} {} with jobs",
                Building::Grandma.name_lower_pluralized(n as _)
            )
        }
    };

    let area = split_area(area);
    let block = Block::bordered()
        .style(Style::new().on_dark_gray())
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
