use super::ModalImpl;
use ratatui::{prelude::*, widgets::Paragraph};

pub fn rename_bakery(name: &str, area: Rect, buf: &mut Buffer) {
    let modal = ModalImpl {
        area,
        buf,
        title: " Rename Bakery ".into(),
        title_bottom: " Submit <Enter> Close <Esc> ".into(),
        screen_percent: (40, 20),
    };

    modal.render(|area, buf, block| {
        Paragraph::new(name).block(block).render(area, buf);
    });
}
