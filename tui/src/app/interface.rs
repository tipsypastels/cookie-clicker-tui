use cookie_clicker_tui_core::Building;
use cookie_clicker_tui_utils::{countdown::Countdown, frames::RefreshClock};
use enum_assoc::Assoc;
use ratatui::style::{Color, Style};
use std::collections::{HashSet, VecDeque};

#[derive(Default)]
pub struct AppInterfaceState {
    even_frame: bool,
    sell_mode: bool,
    pressed_cookie: Countdown<3>,
    flashes: AppFlashes,
}

impl AppInterfaceState {
    pub fn even_frame(&self) -> bool {
        self.even_frame
    }

    pub fn sell_mode(&self) -> bool {
        self.sell_mode
    }

    pub fn pressed_cookie(&self) -> bool {
        self.pressed_cookie.is_running()
    }

    pub fn flash(&self) -> Option<AppFlash> {
        self.flashes.queue.front().copied()
    }

    pub(super) fn toggle_sell_mode(&mut self) {
        self.sell_mode = !self.sell_mode;
    }

    pub(super) fn set_pressed_cookie(&mut self) {
        self.pressed_cookie.run();
    }

    pub(super) fn add_flash(&mut self, flash: AppFlash) {
        self.flashes.add(flash);
    }

    pub(super) fn tick(&mut self) {
        self.even_frame = !self.even_frame;
        self.pressed_cookie.tick();
        self.flashes.tick();
    }
}

#[derive(Default)]
struct AppFlashes {
    queue: VecDeque<AppFlash>,
    contains: HashSet<AppFlash>,
    refresh: RefreshClock<3>,
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
            self.refresh.restart();

            if let Some(front) = self.queue.pop_front() {
                self.contains.remove(&front);
            }
        }
    }
}

#[allow(clippy::enum_variant_names)] // temporary
#[derive(Assoc, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub fn class(self) -> AppFlashClass)]
pub enum AppFlash {
    #[assoc(class = AppFlashClass::Error)]
    CantAffordBuilding(Building),
    #[assoc(class = AppFlashClass::Error)]
    CantAffordUpgrade(usize),
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
}

#[derive(Assoc, Copy, Clone)]
#[func(pub fn title(self) -> &'static str)]
#[func(pub fn style(self) -> Style)]
enum AppFlashClass {
    #[assoc(title = " Error ", style = Style::new().bg(Color::LightRed))]
    Error,
}
