use cookie_clicker_tui_core::{Core, NewsEntry};
use cookie_clicker_tui_utils::refresh::Refresh;
use ratatui::{prelude::*, widgets::Paragraph};
use std::{cell::OnceCell, fmt};

pub struct AppNewsState {
    entry: Option<(NewsEntry, OnceCell<Paragraph<'static>>)>,
    refresh: Refresh,
}

impl AppNewsState {
    pub(super) fn new(core: &Core) -> Self {
        let entry = core.random_news_entry().map(|e| (e, OnceCell::new()));
        let refresh = Refresh::new(30.0);

        Self { entry, refresh }
    }

    pub(super) fn tick(&mut self, core: &Core) {
        if self.refresh.finish() {
            *self = Self::new(core);
        }
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        f: impl FnOnce(NewsEntry) -> Paragraph<'static>,
    ) {
        if let Some((entry, cell)) = self.entry.as_mut() {
            cell.get_or_init(|| f(*entry)).render(area, buf);
        }
    }
}

impl fmt::Debug for AppNewsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppNewsState")
            .field("entry", &self.entry.as_ref().map(|(e, _)| e))
            .field("refresh", &self.refresh)
            .finish()
    }
}
