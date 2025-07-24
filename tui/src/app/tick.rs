#[derive(Default)]
pub struct AppTickState {
    tick_no: u64,
    wrapped: bool,
}

impl AppTickState {
    pub fn tick_no(&self) -> u64 {
        self.tick_no
    }

    pub fn is_very_first(&self) -> bool {
        self.tick_no == 0 && !self.wrapped
    }

    pub(super) fn tick(&mut self) {
        if let Some(tick_no) = self.tick_no.checked_add(1) {
            self.tick_no = tick_no;
        } else {
            self.tick_no = 0;
            self.wrapped = true;
        }
    }
}
