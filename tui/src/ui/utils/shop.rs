use super::{SELECTED_STYLE, num::PrintFloat, style::StyleExt};
use cookie_clicker_tui_core::CostResolved;
use ratatui::prelude::*;
use std::borrow::Cow;

pub struct ShopItemWidget<T> {
    pub item: T,
    pub selected: bool,
    pub affordable: bool,
}

pub trait ShopItemRender {
    fn label(&self) -> Cow<'static, str>;
    fn cost(&self) -> CostResolved;

    fn emoji(&self) -> Option<(&str, Style)> {
        None
    }
}

impl ShopItemWidget<()> {
    pub const HEIGHT: u16 = 1;
}

impl<T: ShopItemRender> Widget for ShopItemWidget<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cols = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(area);

        self.label_line().render(cols[0], buf);
        self.cost_line().render(cols[1], buf);
    }
}

impl<T: ShopItemRender> ShopItemWidget<T> {
    fn label_line(&self) -> Line {
        let mut v = vec![Span::raw(self.item.label())];

        if let Some((emoji, emoji_style)) = self.item.emoji() {
            v.push(Span::styled(emoji, emoji_style));
        }

        Line::from(v).style(
            Style::new()
                .patch_if(self.selected, SELECTED_STYLE)
                .fg_if(!self.affordable, Color::DarkGray),
        )
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!(
                "{} $c",
                match self.item.cost() {
                    CostResolved::Cookies(c) => c.print_float(0, 0),
                }
            ),
            Style::new()
                .patch_if(self.selected, SELECTED_STYLE)
                .fg_if(!self.affordable, Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )
        .right_aligned()
    }
}
