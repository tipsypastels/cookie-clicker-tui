mod building;
mod rename_bakery;
mod upgrade;
mod wrinklers;

use super::UiApp;
use crate::app::{AppListPointee, AppModalState};
use ratatui::{
    prelude::*,
    widgets::{Block, Clear},
};
use std::borrow::Cow;

pub fn modal(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    match app.modal {
        AppModalState::None => {}
        AppModalState::ListItem => match app.list.pointee(app.core) {
            Some(AppListPointee::Building(building)) => {
                building::building(app, building, area, buf)
            }
            Some(AppListPointee::Upgrade(upgrade)) => upgrade::upgrade(upgrade, area, buf),
            None => {}
        },
        AppModalState::RenamingBakery(name) => rename_bakery::rename_bakery(name, area, buf),
        AppModalState::Wrinklers { .. } => wrinklers::wrinklers(app, area, buf),
    }
}

struct ModalImpl<'a> {
    area: Rect,
    buf: &'a mut Buffer,
    title: Cow<'a, str>,
    title_bottom: Cow<'a, str>,
    screen_percent: (u16, u16),
}

impl ModalImpl<'_> {
    fn render(self, f: impl FnOnce(Rect, &mut Buffer, Block)) {
        let area = self.split_area();
        let block = Block::bordered()
            .style(Style::new().white().on_black())
            .border_style(Style::new().white())
            .title(Line::styled(self.title, Modifier::BOLD))
            .title_bottom(Line::styled(self.title_bottom, Modifier::BOLD).right_aligned());

        Clear.render(area, self.buf);
        f(area, self.buf, block)
    }

    fn split_area(&self) -> Rect {
        let (percent_x, percent_y) = self.screen_percent;
        let vert = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]);
        let horiz = Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]);
        horiz.split(vert.split(self.area)[1])[1]
    }
}
