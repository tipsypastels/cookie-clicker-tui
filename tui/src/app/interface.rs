use cookie_clicker_tui_core::{Building, Upgrade};
use cookie_clicker_tui_utils::refresh::{Refresh, RefreshOptionExt};
use enum_assoc::Assoc;
use ratatui::style::{Style, Stylize};
use std::collections::{HashSet, VecDeque};

#[derive(Default, Debug)]
pub struct AppInterfaceState {
    sell_mode: bool,
    pressed_cookie: Option<Refresh>,
    flashes: AppFlashes,
}

impl AppInterfaceState {
    pub fn sell_mode(&self) -> bool {
        self.sell_mode
    }

    pub fn pressed_cookie(&self) -> bool {
        self.pressed_cookie.is_some()
    }

    pub fn flash(&self) -> Option<AppFlash> {
        self.flashes.queue.front().copied()
    }

    pub(super) fn toggle_sell_mode(&mut self) {
        self.sell_mode = !self.sell_mode;
    }

    pub(super) fn set_pressed_cookie(&mut self) {
        self.pressed_cookie = Some(Refresh::new_frames(3.0));
    }

    pub(super) fn add_flash(&mut self, flash: AppFlash) {
        self.flashes.add(flash);
    }

    pub(super) fn tick(&mut self) {
        self.pressed_cookie.finish_and_set_none();
        self.flashes.tick();
    }
}

#[derive(Debug)]
struct AppFlashes {
    queue: VecDeque<AppFlash>,
    contains: HashSet<AppFlash>,
    refresh: Refresh,
}

impl AppFlashes {
    fn add(&mut self, flash: AppFlash) {
        if !self.contains.contains(&flash) {
            self.queue.push_back(flash);
            self.contains.insert(flash);
        }
    }

    fn tick(&mut self) {
        if self.queue.is_empty() {
            return;
        }

        if self.refresh.finish() {
            self.refresh.reset();

            if let Some(front) = self.queue.pop_front() {
                self.contains.remove(&front);
            }
        }
    }
}

impl Default for AppFlashes {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            contains: Default::default(),
            refresh: Refresh::new(3.0),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub fn class(self) -> AppFlashClass)]
pub enum AppFlash {
    #[assoc(class = AppFlashClass::Info)]
    Saved,
    #[assoc(class = AppFlashClass::Warning)]
    WontSaveOverParseError,
    #[assoc(class = AppFlashClass::Info)]
    SugarLumpsUnlocked,
    #[assoc(class = AppFlashClass::Info)]
    ResearchCompleted,
    #[assoc(class = AppFlashClass::Error)]
    CantAffordBuilding(Building),
    #[assoc(class = AppFlashClass::Error)]
    CantAffordUpgrade(Upgrade),
    #[assoc(class = AppFlashClass::Error)]
    CantSellUnownedBuilding(Building),
}

impl AppFlash {
    pub fn title(self) -> &'static str {
        self.class().title()
    }

    pub fn style(self) -> Style {
        self.class().style()
    }

    pub fn border_style(self) -> Style {
        self.class().border_style()
    }
}

#[derive(Assoc, Copy, Clone)]
#[func(fn title(self) -> &'static str)]
#[func(fn style(self) -> Style)]
#[func(fn border_style(self) -> Style)]
enum AppFlashClass {
    #[assoc(title = " Info ", style = Style::new().on_light_blue(), border_style = Style::new().white())]
    Info,
    #[assoc(title = " Warning ", style = Style::new().on_yellow(), border_style = Style::new().black())]
    Warning,
    #[assoc(title = " Error ", style = Style::new().on_light_red(), border_style = Style::new().black())]
    Error,
}
