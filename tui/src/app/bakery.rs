pub struct AppBakery {
    name: Option<Box<str>>,
}

impl AppBakery {
    pub(super) fn new(name: Option<Box<str>>) -> Self {
        Self { name }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub(super) fn set_name(&mut self, name: impl Into<Box<str>>) {
        self.name = Some(name.into());
    }
}
