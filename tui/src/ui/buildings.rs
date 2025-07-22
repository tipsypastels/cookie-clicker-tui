use super::{
    UiApp,
    utils::{
        shop::{ShopItemRender, ShopItemWidget},
        widget::*,
    },
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::{Building, BuildingInfo, Cost};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use std::borrow::Cow;
use tui_widget_list::{ListBuilder, ListView};

pub fn buildings(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let info = app.core.building_info_nth(ctx.index);

        let sell_mode = app.iface.sell_mode();
        let affordable = if sell_mode {
            info.count() > 0
        } else {
            app.core.affordable(info.cost())
        };

        let item = BuildingInfoShopItem { info, sell_mode };
        let widget = ShopItemWidget {
            selected,
            affordable,
            item,
        };

        (widget, ShopItemWidget::HEIGHT)
    });

    let list_view = ListView::new(builder, Building::VARIANT_COUNT);
    let list_state = app.list.state_matching_mut(AppListPane::Buildings);

    let (title, controls, border_style) = if app.iface.sell_mode() {
        (
            " SELL Buildings ",
            " Sell <Enter> Buy Mode <S> Inspect <I> ",
            Style::new().red(),
        )
    } else {
        (
            " Buildings ",
            " Buy <Enter> Sell Mode <S> Inspect <I> ",
            Style::new(),
        )
    };

    let block = Block::bordered()
        .title(Line::styled(title, Modifier::BOLD).centered())
        .title_bottom(Line::styled(controls, Modifier::BOLD).centered())
        .padding(Padding::uniform(1))
        .border_style(border_style);

    list_view
        .block(block)
        .render_stateful_or_default_state(area, buf, list_state);
}

struct BuildingInfoShopItem {
    info: BuildingInfo,
    sell_mode: bool,
}

impl ShopItemRender for BuildingInfoShopItem {
    fn label(&self) -> Cow<'static, str> {
        format!(
            "{} {}",
            self.info.count(),
            self.info.building().name_pluralized(self.info.count() as _)
        )
        .into()
    }

    fn cost(&self) -> Cost {
        if self.sell_mode {
            self.info.sell_cost()
        } else {
            self.info.cost()
        }
    }
}
