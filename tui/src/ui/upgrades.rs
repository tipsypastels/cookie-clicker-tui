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
        let upgrade = app.core.available_upgrades()[ctx.index];
        let affordable = upgrade.cost() <= app.core.cookies();
        let widget = ShopItemWidget {
            selected,
            affordable,
            item: upgrade,
        };

        (widget, ShopItemWidget::HEIGHT)
    });

    let list_view = ListView::new(builder, app.core.available_upgrades().len());
    let list_state = app.list.state_matching_mut(AppListPane::Upgrades);

    let block = Block::bordered()
        .title(Line::styled(" Upgrades ", Modifier::BOLD).centered())
        .title_bottom(Line::styled(" Buy <Enter> Inspect <I> ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    list_view
        .block(block)
        .render_stateful_or_default_state(area, buf, list_state);
}

impl ShopItemRender for Upgrade {
    fn label(&self) -> Cow<'static, str> {
        self.name().into()
    }

    fn cost(&self) -> f64 {
        self.cost()
    }
}
