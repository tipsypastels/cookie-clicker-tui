use cookie_clicker_tui_utils::countdown::Countdown;

#[derive(Default)]
pub struct AppInterfaceState {
    even_frame: bool,
    sell_mode: bool,
    pressed_cookie: Countdown<3>,
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

    pub(super) fn toggle_sell_mode(&mut self) {
        self.sell_mode = !self.sell_mode;
    }

    pub(super) fn set_pressed_cookie(&mut self) {
        self.pressed_cookie.run();
    }

    pub(super) fn tick(&mut self) {
        self.even_frame = !self.even_frame;
        self.pressed_cookie.tick();
    }
}
