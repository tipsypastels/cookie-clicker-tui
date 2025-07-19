use crate::app::AppListPane;

use super::{
    UiApp,
    utils::{num::PrintFloat, widget::StatefulOrDefaultStateWidget},
};
use cookie_clicker_tui_core::Upgrade;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};
use tui_widget_list::{ListBuilder, ListView};

pub fn upgrades(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let upgrade = &app.core.upgrades()[ctx.index];
        let widget = UpgradeWidget { selected, upgrade };

        const HEIGHT: u16 = 2;

        (widget, HEIGHT)
    });

    let list_view = ListView::new(builder, app.core.upgrades().len());
    let list_state = app.list.pane(AppListPane::Upgrades);

    let block = Block::bordered()
        .title(Line::styled(" Upgrades ", Modifier::BOLD))
        .padding(Padding::uniform(1));

    list_view
        .block(block)
        .render_stateful_or_default_state(area, buf, list_state);
}

struct UpgradeWidget<'a> {
    selected: bool,
    upgrade: &'a Upgrade,
}

impl Widget for UpgradeWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Text::from(vec![self.label_line(), self.cost_line()])).render(area, buf);
    }
}

impl UpgradeWidget<'_> {
    fn label_line(&self) -> Line {
        if self.selected {
            Line::styled(self.upgrade.label(), super::SELECTED_STYLE)
        } else {
            Line::raw(self.upgrade.label())
        }
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!("{} cookies", self.upgrade.cost().print_float(0, 0)),
            Modifier::ITALIC,
        )
        .right_aligned()
    }
}
