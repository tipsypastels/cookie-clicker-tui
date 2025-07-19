use enum_assoc::Assoc;
use ratatui::prelude::*;

#[derive(Assoc, Copy, Clone)]
#[func(const fn screen_percent(self) -> (u16, u16))]
pub enum Modal {
    #[assoc(screen_percent = (60, 25))]
    Small,
}

impl Modal {
    pub fn split(self, area: Rect) -> Rect {
        let (percent_x, percent_y) = self.screen_percent();

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

        horiz.split(vert.split(area)[1])[1]
    }
}
