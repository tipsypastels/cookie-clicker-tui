use super::ModalImpl;
use crate::{
    app::AppModalState,
    ui::{
        UiApp,
        utils::{SELECTED_STYLE, num::PrintFloat, style::StyleExt},
    },
};
use cookie_clicker_tui_core::Wrinkler;
use ratatui::prelude::*;
use tui_widget_list::{ListBuilder, ListView};

pub fn wrinklers(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let modal = ModalImpl {
        area,
        buf,
        title: " Wrinklers ".into(),
        title_bottom: " Pop <P> Close <Esc> ".into(),
        screen_percent: (30, 50),
    };

    modal.render(|area, buf, block| {
        let wrinklers = app.core.grandmapocalypse().wrinklers();

        let builder = ListBuilder::new(|ctx| {
            let i = ctx.index;
            let wrinkler = &wrinklers[i];
            let selected = ctx.is_selected;

            let widget = WrinklerWidget {
                i,
                wrinkler,
                selected,
            };

            const HEIGHT: u16 = 1;

            (widget, HEIGHT)
        });

        let list_view = ListView::new(builder, wrinklers.len());
        let list_state = match app.modal {
            AppModalState::Wrinklers { state } => state,
            _ => unreachable!(),
        };

        if let Some(selected) = list_state.selected.as_mut()
            && *selected >= wrinklers.len()
        {
            *selected = wrinklers.len().saturating_sub(1);
        }

        list_view.block(block).render(area, buf, list_state);
    });
}

struct WrinklerWidget<'a> {
    i: usize,
    wrinkler: &'a Wrinkler,
    selected: bool,
}

impl Widget for WrinklerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        self.name_line().render(cols[0], buf);
        self.eaten_line().render(cols[1], buf);
    }
}

impl WrinklerWidget<'_> {
    fn name_line(&self) -> Line {
        Line::styled(
            format!("[^ (▼▼▼) ^] # {}", self.i + 1),
            Style::new().patch_if(self.selected, SELECTED_STYLE),
        )
    }

    // TODO: This should be an upgrade unlockable.
    fn eaten_line(&self) -> Line {
        Line::styled(
            format!("{} cookies", self.wrinkler.eaten().print_float(2, 2)),
            // TODO: selected_if().
            Style::new().patch_if(self.selected, SELECTED_STYLE),
        )
        .right_aligned()
    }
}
