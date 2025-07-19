#[derive(Debug)]
pub struct Countdown<const MAX: u16>(Option<u16>);

impl<const MAX: u16> Countdown<MAX> {
    pub const fn new() -> Self {
        Self(None)
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

#[derive(Debug)]
pub struct CountdownOf<T, const MAX: u16>(Option<(u16, T)>);

impl<T, const MAX: u16> CountdownOf<T, MAX> {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn is_running(&self) -> bool {
        self.0.is_some()
    }

    pub fn value(&self) -> Option<&T> {
        self.0.as_ref().map(|(_, t)| t)
    }

    pub fn run(&mut self, value: T) {
        self.0 = Some((MAX, value))
    }

    pub fn tick(&mut self) {
        self.0 = self
            .0
            .take()
            .and_then(|(n, t)| Some((n.checked_sub(1)?, t)))
    }
}
