#[derive(Debug)]
pub struct Countdown<const MAX: u16>(Option<u16>);

impl<const MAX: u16> Countdown<MAX> {
    pub const fn new() -> Self {
        Self(None)
    }

    pub const fn new_running() -> Self {
        Self(Some(MAX))
    }

    pub fn is_running(&self) -> bool {
        self.0.is_some()
    }

    pub fn run(&mut self) {
        self.0 = Some(MAX);
    }

    pub fn tick(&mut self) {
        self.0 = self.0.and_then(|n| n.checked_sub(1));
    }
}
