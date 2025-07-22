use super::{
    UiApp,
    utils::{
        shop::{ShopItemRender, ShopItemWidget},
        widget::*,
    },
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::Upgrade;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use std::borrow::Cow;
use tui_widget_list::{ListBuilder, ListView};

pub fn upgrades(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let upgrade = &app.core.upgrades()[ctx.index];
        let affordable = upgrade.cost() <= app.core.cookies();
        let widget = ShopItemWidget {
            selected,
            affordable,
            item: upgrade,
        };

        (widget, ShopItemWidget::HEIGHT)
    });

    let list_view = ListView::new(builder, app.core.upgrades().len());
    let list_state = app.list.state_matching_mut(AppListPane::Upgrades);

    let (title, border_style) = if app.iface.sell_mode() {
        (" SELL Upgrades ", Style::new().red())
    } else {
        (" Upgrades ", Style::new())
    };

    let block = Block::bordered()
        .title(Line::styled(title, Modifier::BOLD).centered())
        .title_bottom(Line::styled(" Buy <Enter> Inspect <I> ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1))
        .border_style(border_style);

    list_view
        .block(block)
        .render_stateful_or_default_state(area, buf, list_state);
}

impl ShopItemRender for &Upgrade {
    fn label(&self) -> Cow<'static, str> {
        Upgrade::label(self).into()
    }

    fn cost(&self) -> f64 {
        Upgrade::cost(self)
    }
}
