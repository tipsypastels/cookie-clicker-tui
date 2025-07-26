use super::UiApp;
use cookie_clicker_tui_core::GoldenCookie;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};

pub fn golden_cookies(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    for cookie in app.core.golden_cookies().iter() {
        golden_cookie(cookie, area, buf);
    }
}

fn golden_cookie(cookie: &GoldenCookie, area: Rect, buf: &mut Buffer) {
    const WIDTH: u16 = 16;
    const HEIGHT: u16 = 8;
    const LOGO: &str = "⠀⠀⣠⣶⣖⣲⣤⣤⣤⣄⡀⠀⠀⠀
⢰⠏⡴⠲⣄⠀⠀⠱⢤⣀⣈⣷⡄⠀
⢸⠀⠸⣀⡼⠀⠀⠀⣀⣀⡀⠀⠸⡇⠀⠀⠀⠀⠀⠀⠀
⠸⡒⡆⠀⠀⠀⠀⠀⣎⠀⠀⠉⡳⡇
⠀⠛⢥⡤⠒⢄⠀⠀⠈⢓⣦⣾⠅⠀
⠀⠀⠀⠉⠭⠭⠭⠵⠿⠛⠉⠀⠀⠀";

    let area = Rect {
        x: ((area.width as f64 * cookie.x()).trunc() as u16).clamp(0, area.width - WIDTH - 1),
        y: ((area.height as f64 * cookie.x()).trunc() as u16).clamp(0, area.height - HEIGHT - 1),
        width: WIDTH,
        height: HEIGHT,
    };

    let lines = LOGO.lines().map(Line::raw).collect::<Vec<_>>();
    let block = Block::bordered().black().on_yellow().title_bottom(
        Line::styled(format!(" Click <{}> ", cookie.ch()), Modifier::BOLD).right_aligned(),
    );

    Clear.render(area, buf);
    Paragraph::new(lines).block(block).render(area, buf);
}
