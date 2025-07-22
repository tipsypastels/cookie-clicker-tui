use cookie_clicker_tui_core::Building;
use cookie_clicker_tui_utils::countdown::{Countdown, CountdownOf};

#[derive(Default)]
pub struct AppInterfaceState {
    even_frame: bool,
    sell_mode: bool,
    pressed_cookie: Countdown<3>,
    insufficient_cookies: Countdown<10>,
    tried_to_sell_unowned_building: CountdownOf<Building, 10>,
    tried_to_sell_unowned_upgrade: CountdownOf<usize, 10>,
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

    pub fn insufficient_cookies(&self) -> bool {
        self.insufficient_cookies.is_running()
    }

    pub fn tried_to_sell_unowned_building(&self) -> Option<Building> {
        self.tried_to_sell_unowned_building.value().copied()
    }

    pub fn tried_to_sell_unowned_upgrade(&self) -> Option<usize> {
        self.tried_to_sell_unowned_upgrade.value().copied()
    }

    pub(super) fn toggle_sell_mode(&mut self) {
        self.sell_mode = !self.sell_mode;
    }

    pub(super) fn set_pressed_cookie(&mut self) {
        self.pressed_cookie.run();
    }

    pub(super) fn set_insufficient_cookies(&mut self) {
        self.insufficient_cookies.run();
    }

    pub(super) fn set_tried_to_sell_unowned_building(&mut self, building: Building) {
        self.tried_to_sell_unowned_building.run(building);
    }

    pub(super) fn set_tried_to_sell_unowned_upgrade(&mut self, index: usize) {
        self.tried_to_sell_unowned_upgrade.run(index);
    }

    pub(super) fn tick(&mut self) {
        self.even_frame = !self.even_frame;
        self.pressed_cookie.tick();
        self.insufficient_cookies.tick();
        self.tried_to_sell_unowned_building.tick();
        self.tried_to_sell_unowned_upgrade.tick();
    }
}
