use crossterm::event::KeyEvent;
use enum_fun::Name;

#[derive(Default)]
pub struct AppDebugState {
    view: Option<AppDebugView>,
    latest_key: Option<KeyEvent>,
}

#[derive(Name, Default, Copy, Clone)]
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

    pub fn advance(&mut self) {
        self.view = Some(match self.view {
            Some(view) => view.next(),
            None => AppDebugView::default(),
        })
    }
}

impl AppDebugView {
    fn next(self) -> Self {
        match self {
            Self::Cookies => Self::Cps,
            Self::Cps => Self::Buildings,
            Self::Buildings => Self::BuildingsFlags,
            Self::BuildingsFlags => Self::AvailableUpgrades,
            Self::AvailableUpgrades => Self::OwnedUpgrades,
            Self::OwnedUpgrades => Self::Achievements,
            Self::Achievements => Self::Milk,
            Self::Milk => Self::SugarLumps,
            Self::SugarLumps => Self::Research,
            Self::Research => Self::Grandmapocalypse,
            Self::Grandmapocalypse => Self::Ticker,
            Self::Ticker => Self::List,
            Self::List => Self::Keypress,
            Self::Keypress => Self::Cookies,
        }
    }
}
