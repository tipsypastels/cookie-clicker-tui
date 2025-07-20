use super::{
    SELECTED_STYLE, UiApp,
    utils::{num::PrintFloat, style::StyleExt, widget::StatefulOrDefaultStateWidget},
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::Upgrade;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use tui_widget_list::{ListBuilder, ListView};

pub fn upgrades(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let upgrade = &app.core.upgrades()[ctx.index];
        let affordable = upgrade.cost() <= app.core.cookies();
        let widget = UpgradeWidget {
            selected,
            affordable,
            upgrade,
        };

        const HEIGHT: u16 = 1;

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
    affordable: bool,
    upgrade: &'a Upgrade,
}

impl Widget for UpgradeWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cols = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(area);

        self.label_line().render(cols[0], buf);
        self.cost_line().render(cols[1], buf);
    }
}

impl UpgradeWidget<'_> {
    fn label_line(&self) -> Line {
        Line::styled(
            self.upgrade.label(),
            Style::new().patch_if(self.selected, SELECTED_STYLE),
        )
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!("{} $c", self.upgrade.cost().print_float(0, 0)),
            Style::new()
                .patch_if(self.selected, SELECTED_STYLE)
                .fg_if(!self.affordable, Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )
        .right_aligned()
    }
}
