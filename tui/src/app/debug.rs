use crossterm::event::KeyEvent;
use enum_fun::{Name, Variants};

#[derive(Default)]
pub struct AppDebugState {
    view: Option<AppDebugView>,
    latest_key: Option<KeyEvent>,
}

#[derive(Name, Variants, Default, Copy, Clone)]
#[name(base = "title case")]
pub enum AppDebugView {
    #[default]
    Cookies,
    Cps,
    Buildings,
    BuildingsFlags,
    AvailableUpgrades,
    OwnedUpgrades,
    Achievements,
    Milk,
    SugarLumps,
    Research,
    Grandmapocalypse,
    Ticker,
    List,
    Keypress,
    Save,
}

impl AppDebugState {
    pub fn view(&self) -> Option<AppDebugView> {
        self.view
    }

    pub fn is_open(&self) -> bool {
        self.view.is_some()
    }

    pub fn latest_key_event(&self) -> Option<KeyEvent> {
        self.latest_key
    }

    pub fn set_latest_key_event(&mut self, key: KeyEvent) {
        self.latest_key = Some(key);
    }

    pub fn close(&mut self) {
        self.view = None;
    }

    pub fn backward(&mut self) {
        self.view = Some(match self.view {
            Some(view) => view.prev(),
            None => AppDebugView::default(),
        })
    }

    pub fn forward(&mut self) {
        self.view = Some(match self.view {
            Some(view) => view.next(),
            None => AppDebugView::default(),
        })
    }
}

impl AppDebugView {
    fn prev(self) -> Self {
        Self::VARIANTS[if self as usize == 0 {
            Self::VARIANT_COUNT - 1
        } else {
            self as usize - 1
        }]
    }

    fn next(self) -> Self {
        Self::VARIANTS[if self as usize + 1 == Self::VARIANT_COUNT {
            0
        } else {
            self as usize + 1
        }]
    }
}
