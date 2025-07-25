use super::{UiApp, utils::num::PrintFloat};
use cookie_clicker_tui_core::MilkFlavor;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};
use std::borrow::Cow;

const LOGO_HEIGHT: usize = 15;
const LOGO_PADDING_LEFT: usize = 2;
const LOGO: &str = r#"  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡴⠚⣉⡙⠲⠦⠤⠤⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
  ⠀⠀⠀⠀⠀⠀⢀⣴⠛⠉⠉⠀⣾⣷⣿⡆⠀⠀⠀⠐⠛⠿⢟⡲⢦⡀⠀⠀⠀⠀
  ⠀⠀⠀⠀⣠⢞⣭⠎⠀⠀⠀⠀⠘⠛⠛⠀⠀⢀⡀⠀⠀⠀⠀⠈⠓⠿⣄⠀⠀⠀
  ⠀⠀⠀⡜⣱⠋⠀⠀⣠⣤⢄⠀⠀⠀⠀⠀⠀⣿⡟⣆⠀⠀⠀⠀⠀⠀⠻⢷⡄⠀
  ⠀⢀⣜⠜⠁⠀⠀⠀⢿⣿⣷⣵⠀⠀⠀⠀⠀⠿⠿⠿⠀⠀⣴⣶⣦⡀⠀⠰⣹⡆
  ⢀⡞⠆⠀⣀⡀⠀⠀⠘⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣶⠇⠀⢠⢻⡇
  ⢸⠃⠘⣾⣏⡇⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⣠⣤⣤⡉⠁⠀⠀⠈⠫⣧
  ⡸⡄⠀⠘⠟⠀⠀⠀⠀⠀⠀⣰⣿⣟⢧⠀⠀⠀⠀⠰⡿⣿⣿⢿⠀⠀⣰⣷⢡⢸
  ⣿⡇⠀⠀⠀⣰⣿⡻⡆⠀⠀⠻⣿⣿⣟⠀⠀⠀⠀⠀⠉⠉⠉⠀⠀⠘⢿⡿⣸⡞
  ⠹⣽⣤⣤⣤⣹⣿⡿⠇⠀⠀⠀⠀⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡔⣽⠀
  ⠀⠙⢻⡙⠟⣹⠟⢷⣶⣄⢀⣴⣶⣄⠀⠀⠀⠀⠀⢀⣤⡦⣄⠀⠀⢠⣾⢸⠏⠀
  ⠀⠀⠘⠀⠀⠀⠀⠀⠈⢷⢼⣿⡿⡽⠀⠀⠀⠀⠀⠸⣿⣿⣾⠀⣼⡿⣣⠟⠀⠀
  ⠀⠀⠀⠀⠀⠀⠀⠀⢠⡾⣆⠑⠋⠀⢀⣀⠀⠀⠀⠀⠈⠈⢁⣴⢫⡿⠁⠀⠀⠀
  ⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⣧⣄⡄⠴⣿⣶⣿⢀⣤⠶⣞⣋⣩⣵⠏⠀⠀⠀⠀⠀
  ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⣿⢯⣭⣭⣯⣯⣥⡵⠿⠟⠛⠉⠉⠀⠀⠀⠀⠀⠀⠀"#;

pub fn cookies(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let mut lines = Vec::with_capacity(LOGO_HEIGHT + 3);

    cookie_count(app, &mut lines);
    cps_count(app, &mut lines);
    lines.push(Line::default());
    logo(app, &mut lines);

    let title: Cow<str> = match app.bakery.name() {
        Some(name) => format!(" {name} ").into(),
        None => " Cookies ".into(),
    };

    let block = Block::bordered()
        .title(Line::styled(title, Modifier::BOLD).centered())
        .title_bottom(Line::styled(" Click <Space> Rename <R> ", Modifier::BOLD).centered());

    let block_area = block.inner(area);

    milk_wave(app, block_area, buf);
    sugar_lump(app, block_area, buf);

    Paragraph::new(Text::from(lines))
        .centered()
        .block(block)
        .render(area, buf);
}

fn cookie_count(app: &mut UiApp, lines: &mut Vec<Line>) {
    lines.push(Line::styled(
        format!("{}", app.core.cookies().print_float(0, 2)),
        Modifier::BOLD,
    ));
}

fn cps_count(app: &mut UiApp, lines: &mut Vec<Line>) {
    lines.push(Line::styled(
        format!("(per second: {})", app.core.cps().print_float(1, 2)),
        Modifier::ITALIC,
    ));
}

fn logo(app: &mut UiApp, lines: &mut Vec<Line>) {
    for line_text in LOGO.lines() {
        let line_text = if app.iface.pressed_cookie() && app.tick.tick_no() % 2 == 0 {
            &line_text[LOGO_PADDING_LEFT..]
        } else {
            line_text
        };

        // style overwrites colors from milk
        lines.push(Line::styled(line_text, Color::White))
    }
}

fn milk_wave(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    const WAVE: &str = "~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ";

    let milk = app.core.milk();

    if milk.is_empty() {
        return;
    }

    let percent = ((milk.ratio() * 100.0) as u16).min(100);
    let style = milk_style(milk.flavor());

    let area = Layout::vertical([
        Constraint::Percentage(100 - percent),
        Constraint::Percentage(percent),
    ])
    .split(area)[1];

    Text::styled(WAVE, style).render(area, buf);
}

fn milk_style(flavor: MilkFlavor) -> Style {
    match flavor {
        MilkFlavor::Plain => Style::new().white(),
        MilkFlavor::Chocolate => Style::new().dark_gray(),
        MilkFlavor::Raspberry => Style::new().red(),
        MilkFlavor::Orange => Style::new().yellow(),
        MilkFlavor::Caramel => Style::new().gray(),
        MilkFlavor::Banana => Style::new().yellow(),
        MilkFlavor::Lime => Style::new().light_green(),
        MilkFlavor::Blueberry => Style::new().cyan(),
        MilkFlavor::Strawberry => Style::new().light_red(),
        MilkFlavor::Vanilla => Style::new().light_yellow(),
        MilkFlavor::Honey => Style::new().yellow(),
        MilkFlavor::Coffee => Style::new().dark_gray(),
        MilkFlavor::Tea => Style::new().gray(),
        MilkFlavor::Coconut => Style::new().white(),
        MilkFlavor::Cherry => Style::new().red(),
        MilkFlavor::Spiced => Style::new().white().italic(),
        MilkFlavor::Maple => Style::new().gray(),
        MilkFlavor::Mint => Style::new().light_cyan(),
        MilkFlavor::Licorice => Style::new().black().italic(),
        MilkFlavor::Rose => Style::new().light_magenta(),
        MilkFlavor::Dragonfruit => Style::new().magenta(),
        MilkFlavor::Melon => Style::new().green(),
        MilkFlavor::Blackcurrant => Style::new().magenta().italic(),
        MilkFlavor::Peach => Style::new().light_magenta().italic(),
        MilkFlavor::Hazelnut => Style::new().white().italic(),
    }
}

fn sugar_lump(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let sugar_lumps = app.core.sugar_lumps();

    if !sugar_lumps.unlocked() {
        return;
    }

    let count = sugar_lumps.count();
    let text: Cow<str> = if count == 0 {
        "𖧋  ".into() // spaces used as padding :(
    } else {
        format!("{count} ●  ").into()
    };

    Line::raw(text).right_aligned().render(area, buf);
}
