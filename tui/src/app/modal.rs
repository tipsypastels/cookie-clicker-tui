// will probably become an enum with variants later
#[derive(Default, Copy, Clone)]
pub struct AppModalState {
    list_item: bool,
}

impl AppModalState {
    pub fn is_open(&self) -> bool {
        self.list_item
    }

    pub fn is_list_item(&self) -> bool {
        self.list_item
    }

    pub(super) fn toggle(&mut self) {
        self.list_item = !self.list_item;
    }

    pub(super) fn close(&mut self) {
        self.list_item = false;
    }
}
