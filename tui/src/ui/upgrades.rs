use super::{
    UiApp,
    utils::{
        shop::{ShopItemRender, ShopItemWidget},
        widget::*,
    },
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::{Cost, Upgrade};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use std::borrow::Cow;
use tui_widget_list::{ListBuilder, ListView};

pub fn upgrades(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let upgrades = app.core.available_upgrades();

    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let upgrade = upgrades[ctx.index];
        let affordable = app.core.affordable(upgrade.cost());
        let widget = ShopItemWidget {
            selected,
            affordable,
            item: upgrade,
        };

        (widget, ShopItemWidget::HEIGHT)
    });

    let list_view = ListView::new(builder, app.core.available_upgrades().len());
    let mut list_state = app.list.state_matching_mut(AppListPane::Upgrades);

    // This force clamps the list to the upgrade length. It would be nice to not do this
    // while rendering, but I can't always guarantee that upgrade length will change in
    // response to a key input in the future? If this is needed for other lists, then
    // refactor the list state itself to include it.
    if let Some(selected) = list_state.as_mut().and_then(|s| s.selected.as_mut())
        && *selected >= upgrades.len()
    {
        *selected = upgrades.len().saturating_sub(1);
    }

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

    fn cost(&self) -> Cost {
        self.cost()
    }
}
