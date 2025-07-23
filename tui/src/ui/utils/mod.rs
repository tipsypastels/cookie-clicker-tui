pub mod num;
pub mod shop;
pub mod style;
pub mod upgrade;
pub mod widget;

use ratatui::prelude::*;

pub const SELECTED_STYLE: Style = Style::new()
    .bg(Color::White)
    .fg(Color::Black)
    .add_modifier(Modifier::BOLD);
