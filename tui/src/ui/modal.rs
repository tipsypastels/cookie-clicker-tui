use super::{UiApp, utils::num::PrintFloat};
use crate::app::{AppListPane, AppModalState};
use cookie_clicker_tui_core::Building;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph, Wrap},
};
use std::borrow::Cow;

const SCREEN_PERCENT: (u16, u16) = (60, 25);

pub fn modal(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    match app.modal {
        AppModalState::Closed => {}
        AppModalState::Debug(message) => render_debug(message, area, buf),
        AppModalState::ListItem => render_list_item(app, area, buf),
    }
}

fn render_debug(message: &str, area: Rect, buf: &mut Buffer) {
    render_outer(area, buf, " Debug ", |area, buf, block| {
        Paragraph::new(Text::raw(message))
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buf);
    });
}

fn render_list_item(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let Some((pane, index)) = app.list.selected() else {
        return;
    };
    match pane {
        AppListPane::Buildings => render_building(app, index, area, buf),
        AppListPane::Upgrades => render_upgrade(app, index, area, buf),
    }
}

fn render_building(app: &mut UiApp, index: usize, area: Rect, buf: &mut Buffer) {
    let Some(building) = Building::nth(index) else {
        return;
    };
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

        Paragraph::new(vec![cps_per_line, cps_line, cps_percent_line])
            .block(block)
            .render(area, buf);
    });
}

fn render_upgrade(app: &mut UiApp, index: usize, area: Rect, buf: &mut Buffer) {
    let Some(upgrade) = app.core.upgrades().get(index) else {
        return;
    };

    let title = format!(" {} ", upgrade.label());

    render_outer(area, buf, title, |area, buf, block| {
        // TODO: Consider making more of upgrade public
        // so that tokens in here can be bolded?
        let description_line = Line::raw(format!("• {}", upgrade.description()));

        Paragraph::new(vec![description_line])
            .block(block)
            .render(area, buf);
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
        .title_bottom(Line::styled(" Close <I> ", Modifier::BOLD).right_aligned())
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
