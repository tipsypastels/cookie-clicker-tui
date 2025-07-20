use super::{SELECTED_STYLE, num::PrintFloat, style::StyleExt};
use ratatui::prelude::*;
use std::borrow::Cow;

pub struct ShopItemWidget<T> {
    pub item: T,
    pub selected: bool,
    pub affordable: bool,
}

pub trait ShopItemRender {
    fn label(&self) -> Cow<'static, str>;
    fn cost(&self) -> f64;
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
        Line::styled(
            self.item.label(),
            Style::new().patch_if(self.selected, SELECTED_STYLE),
        )
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!("{} $c", self.item.cost().print_float(0, 0),),
            Style::new()
                .patch_if(self.selected, SELECTED_STYLE)
                .fg_if(!self.affordable, Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )
        .right_aligned()
    }
}
