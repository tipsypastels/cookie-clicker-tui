use tui_widget_list::ListState;

#[derive(Default)]
pub enum AppModalState {
    #[default]
    None,
    ListItem,
    Wrinklers {
        state: ListState,
    },
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

    pub(super) fn set_wrinklers(&mut self) {
        *self = Self::Wrinklers {
            state: ListState::default(),
        };
    }

    pub(super) fn close(&mut self) {
        *self = Self::None;
    }
}
