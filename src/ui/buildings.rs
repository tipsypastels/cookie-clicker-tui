use super::util::{num::PrintFloat, widget::*};
use crate::app::{App, Building, ListStatePane};
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};
use tui_widget_list::{ListBuilder, ListView};

pub fn buildings(app: &mut App, area: Rect, buf: &mut Buffer) {
    let app_parts = app.deconstruct_for_rendering();
    let buildings = app_parts.buildings;

    let builder = ListBuilder::new(|ctx| {
        let selected = ctx.is_selected;
        let building = Building::ALL[ctx.index];
        let count = buildings.count(building);
        let cost = buildings.cost(building);
        // TODO: Get this off buildings to take account of upgrades.
        let cps = building.base_cps();

        let widget = BuildingWidget {
            selected,
            building,
            count,
            cost,
            cps,
        };

        const HEIGHT: u16 = 3;

        (widget, HEIGHT)
    });

    let block = Block::bordered()
        .title(Line::styled(" Buildings ", Modifier::BOLD).centered())
        .padding(Padding::uniform(1));

    let list_view = ListView::new(builder, Building::ALL.len());
    let list_state = app_parts.list.state_for_pane(ListStatePane::Buildings);

    list_view
        .block(block)
        .render_stateful_or_default_state(area, buf, list_state);
}

struct BuildingWidget {
    selected: bool,
    building: Building,
    count: u16,
    cost: f64,
    cps: f64,
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
            self.count,
            self.building.name_pluralized(self.count as _)
        );

        if self.selected {
            Line::styled(label, super::SELECTED_STYLE)
        } else {
            Line::raw(label)
        }
    }

    fn cost_line(&self) -> Line {
        Line::styled(
            format!("{} cookies", self.cost.print_float(0, 0)),
            Modifier::ITALIC,
        )
        .right_aligned()
    }

    fn cps_line(&self) -> Line {
        Line::styled(
            format!("{} cps", self.cps.print_float(1, 0)),
            Modifier::ITALIC,
        )
        .right_aligned()
    }
}
