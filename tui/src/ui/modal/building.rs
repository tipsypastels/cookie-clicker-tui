use super::{
    super::{UiApp, utils::num::PrintFloat},
    ModalImpl,
};
use cookie_clicker_tui_core::Building;
use ratatui::{prelude::*, widgets::Paragraph};

pub fn building(app: &mut UiApp, building: Building, area: Rect, buf: &mut Buffer) {
    let info = app.core.building_info(building);
    let count = info.count();
    let name = building.name_pluralized(count as _);
    let title = format!(" {count} {name} ");

    let modal = ModalImpl {
        area,
        buf,
        title: title.into(),
        title_bottom: " Close <Esc> ".into(),
        screen_percent: (60, 31),
    };

    modal.render(|area, buf, block| {
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
