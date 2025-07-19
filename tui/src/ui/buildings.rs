use super::{
    UiApp,
    utils::{num::PrintFloat, widget::*},
};
use crate::app::AppListPane;
use cookie_clicker_tui_core::{Building, BuildingInfo};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};
use tui_widget_list::{ListBuilder, ListView};

pub fn buildings(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let info = app.core.building_info_nth(ctx.index);
        let widget = BuildingWidget { selected, info };

        const HEIGHT: u16 = 3;

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
    info: BuildingInfo,
}

impl Widget for BuildingWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Text::from(vec![
            self.label_line(),
            self.cost_line(),
            self.cps_line(),
        ]))
        .render(area, buf);
    }
}

impl BuildingWidget {
    fn label_line(&self) -> Line {
        let label = format!(
            "{} {}",
            self.info.count(),
            self.info.building().name_pluralized(self.info.count() as _)
        );

        if self.selected {
            Line::styled(label, super::SELECTED_STYLE)
        } else {
            Line::raw(label)
        }
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!("{} cookies", self.info.cost().print_float(0, 0)),
            Modifier::ITALIC,
        )
        .right_aligned()
    }

    fn cps_line(&self) -> Line {
        Line::styled(
            format!("{} cps", self.info.cps().print_float(1, 0)),
            Modifier::ITALIC,
        )
        .right_aligned()
    }
}
