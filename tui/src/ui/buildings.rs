use super::{
    SELECTED_STYLE, UiApp,
    utils::{num::PrintFloat, style::StyleExt, widget::*},
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::{Building, BuildingInfo};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use tui_widget_list::{ListBuilder, ListView};

pub fn buildings(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let info = app.core.building_info_nth(ctx.index);
        let affordable = info.cost() <= app.core.cookies();
        let widget = BuildingWidget {
            selected,
            affordable,
            info,
        };

        const HEIGHT: u16 = 1;

        (widget, HEIGHT)
    });

    let list_view = ListView::new(builder, Building::VARIANT_COUNT);
    let list_state = app.list.pane(AppListPane::Buildings);

    let block = Block::bordered()
        .title(Line::styled(" Buildings ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    list_view
        .block(block)
        .render_stateful_or_default_state(area, buf, list_state);
}

struct BuildingWidget {
    selected: bool,
    affordable: bool,
    info: BuildingInfo,
}

impl Widget for BuildingWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cols = Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(area);

        self.label_line().render(cols[0], buf);
        self.cost_line().render(cols[1], buf);
    }
}

impl BuildingWidget {
    fn label_line(&self) -> Line {
        Line::styled(
            format!(
                "{} {}",
                self.info.count(),
                self.info.building().name_pluralized(self.info.count() as _)
            ),
            Style::new().patch_if(self.selected, SELECTED_STYLE),
        )
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!("{} $c", self.info.cost().print_float(0, 0)),
            Style::new()
                .patch_if(self.selected, SELECTED_STYLE)
                .fg_if(!self.affordable, Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )
        .right_aligned()
    }
}
