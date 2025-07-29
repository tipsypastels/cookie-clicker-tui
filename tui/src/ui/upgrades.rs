use super::{
    UiApp,
    utils::shop::{ShopItemRender, ShopItemWidget},
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::{Core, CostResolved, Upgrade};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use std::borrow::Cow;
use tui_widget_list::{ListBuilder, ListView};

pub fn upgrades(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let upgrades = app.core.available_upgrades();
    let (list_selected, list_state) = app.list.get_for_render(AppListPane::Upgrades, app.core);

    let builder = ListBuilder::new(|ctx| {
        let selected = list_selected && ctx.is_selected;
        let upgrade = upgrades[ctx.index];
        let affordable = app.core.affordable(upgrade.cost());

        let item = UpgradeShopItem {
            core: app.core,
            upgrade,
        };

        let widget = ShopItemWidget {
            selected,
            affordable,
            item,
        };

        (widget, ShopItemWidget::HEIGHT)
    });

    let list_view = ListView::new(builder, upgrades.len());

    let block = Block::bordered()
        .title(Line::styled(" Upgrades ", Modifier::BOLD).centered())
        .title_bottom(Line::styled(" Buy <Enter> Inspect <I> ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    list_view.block(block).render(area, buf, list_state);
}

struct UpgradeShopItem<'a> {
    core: &'a Core,
    upgrade: Upgrade,
}

impl ShopItemRender for UpgradeShopItem<'_> {
    fn label(&self) -> Cow<'static, str> {
        self.upgrade.name().into()
    }

    fn cost(&self) -> CostResolved {
        self.core.resolve_cost(self.upgrade.cost())
    }
}
