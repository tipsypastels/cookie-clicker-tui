use super::{
    UiApp,
    utils::{num::PrintFloat, upgrade::print_upgrade_effect_info},
};
use crate::app::AppListPointee;
use cookie_clicker_tui_core::{Building, Upgrade};
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};
use std::borrow::Cow;

const SCREEN_PERCENT: (u16, u16) = (60, 31);

pub fn modal(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    if app.modal.is_list_item() {
        render_list_item(app, area, buf);
    }
}

fn render_list_item(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    match app.list.pointee(app.core) {
        Some((_, AppListPointee::Building(building))) => render_building(app, building, area, buf),
        Some((_, AppListPointee::Upgrade(upgrade))) => render_upgrade(upgrade, area, buf),
        None => {}
    }
}

fn render_building(app: &mut UiApp, building: Building, area: Rect, buf: &mut Buffer) {
    let info = app.core.building_info(building);
    let count = info.count();
    let name = building.name_pluralized(count as _);
    let title = format!(" {count} {name} ");

    render_outer(area, buf, title, |area, buf, block| {
        let cps = info.cps();
        let cps_per = if count == 0 { 0.0 } else { cps / count as f64 };
        let cps_percent = if cps == 0.0 {
            0.0
        } else {
            cps / app.core.cps() * 100.0
        };

        let cps_per_line = Line::from(vec![
            Span::raw("• producing "),
            Span::styled(format!("{}", cps_per.print_float(1, 0)), Modifier::BOLD),
            Span::raw(" cookies per second "),
            Span::styled("each", Modifier::BOLD),
        ]);

        let cps_line = Line::from(vec![
            Span::raw("• producing "),
            Span::styled(format!("{}", cps.print_float(1, 0)), Modifier::BOLD),
            Span::raw(" cookies per second "),
            Span::styled("total", Modifier::BOLD),
        ]);

        let cps_percent_line = Line::from(vec![
            Span::raw("• producing "),
            Span::styled(
                format!("{}%", cps_percent.print_float(1, 1)),
                Modifier::BOLD,
            ),
            Span::raw(" of your "),
            Span::styled("total", Modifier::BOLD),
            Span::raw(" cookies per second"),
        ]);

        let cookies_all_time_line = Line::from(vec![
            Span::raw("• produced "),
            Span::styled(
                format!("{}", info.cookies_all_time().print_float(0, 0)),
                Modifier::BOLD,
            ),
            Span::raw(" cookies in all time"),
        ]);

        Paragraph::new(vec![
            cps_per_line,
            cps_line,
            cps_percent_line,
            cookies_all_time_line,
        ])
        .block(block)
        .render(area, buf);
    });
}

fn render_upgrade(upgrade: Upgrade, area: Rect, buf: &mut Buffer) {
    let title = format!(" {} ", upgrade.name());

    render_outer(area, buf, title, |area, buf, block| {
        let mut lines = Vec::new();
        let info = upgrade.effect_info();

        print_upgrade_effect_info(info, &mut lines);
        Paragraph::new(lines).block(block).render(area, buf);
    });
}

fn render_outer<'a>(
    area: Rect,
    buf: &mut Buffer,
    title: impl Into<Cow<'a, str>>,
    f: impl FnOnce(Rect, &mut Buffer, Block),
) {
    let area = split_area(area);
    let block = Block::bordered()
        .border_style(Style::new().black())
        .title(Line::styled(title, Modifier::BOLD))
        .title_bottom(Line::styled(" Close <Esc> ", Modifier::BOLD).right_aligned())
        .style(Style::new().bg(Color::DarkGray));

    Clear.render(area, buf);
    f(area, buf, block);
}

fn split_area(area: Rect) -> Rect {
    let (percent_x, percent_y) = SCREEN_PERCENT;
    let vert = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ]);
    let horiz = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]);
    horiz.split(vert.split(area)[1])[1]
}
