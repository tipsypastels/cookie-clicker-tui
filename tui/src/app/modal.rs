#[derive(Default)]
pub enum AppModalState {
    #[default]
    None,
    ListItem,
    RenamingBakery(String),
}

impl AppModalState {
    pub fn is_open(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub(super) fn toggle_list_item(&mut self) {
        match self {
            Self::None => {
                *self = Self::ListItem;
            }
            Self::ListItem => {
                *self = Self::None;
            }
            _ => {}
        }
    }

    pub(super) fn set_renaming_bakery(&mut self) {
        *self = Self::RenamingBakery(String::new());
    }

    pub(super) fn close(&mut self) {
        *self = Self::None;
    }
}
