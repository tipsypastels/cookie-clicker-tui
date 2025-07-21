use crate::{
    event::{Event, Events, REVERSE_MODIFIER},
    storage::Storage,
};
use anyhow::{Context, Result};
use cookie_clicker_tui_core::{Building, Core};
use cookie_clicker_tui_utils::countdown::{Countdown, CountdownOf};
use crossterm::event::KeyEvent;
use enum_fun::Name;
use ratatui::DefaultTerminal;
use tui_widget_list::ListState;

pub struct App {
    storage: Storage,
    core: Core,
    list: AppListState,
    countdown: AppCountdownState,
    modal: AppModalState,
    debug: Option<AppDebugView>,
    debug_latest_key: Option<KeyEvent>,
    events: Events,
    quit: bool,
}

pub struct AppListState {
    buildings: ListState,
    upgrades: ListState,
    pane: AppListPane,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum AppListPane {
    #[default]
    Buildings,
    Upgrades,
}

pub struct AppCountdownState {
    just_pressed_cookie: Countdown<3>,
    error_insufficient_cookies: Countdown<10>,
    error_tried_to_sell_unowned_building: CountdownOf<Building, 10>,
}

#[derive(Default, Copy, Clone)]
pub enum AppModalState {
    ListItem,
    #[default]
    Closed,
}

#[derive(Name, Default, Copy, Clone)]
#[name(base = "title case")]
pub enum AppDebugView {
    #[default]
    Cookies,
    Buildings,
    Upgrades,
    Achievements,
    Milk,
    Ticker,
    Keypress,
}

impl App {
    pub fn new(storage: Storage, core: Core) -> Self {
        Self {
            storage,
            core,
            list: AppListState {
                buildings: ListState::default(),
                upgrades: ListState::default(),
                pane: AppListPane::default(),
            },
            countdown: AppCountdownState {
                just_pressed_cookie: Countdown::new(),
                error_insufficient_cookies: Countdown::new(),
                error_tried_to_sell_unowned_building: CountdownOf::new(),
            },
            modal: AppModalState::default(),
            debug: None,
            debug_latest_key: None,
            events: Events::new(),
            quit: false,
        }
    }

    pub async fn run(mut self, term: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            self.draw(term)?;

            match self.events.next().await? {
                Event::Tick => {
                    self.tick().await?;
                }
                Event::Term(crossterm::event::Event::Key(event)) if event.is_press() => {
                    self.handle_key_event(event).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, event: KeyEvent) -> Result<()> {
        use crossterm::event::KeyCode;

        self.debug_latest_key = Some(event);

        match event.code {
            KeyCode::Up => {
                self.list.up();
            }
            KeyCode::Down => {
                self.list.down();
            }
            KeyCode::Left => {
                self.list.left(&self.core);
            }
            KeyCode::Right => {
                self.list.right(&self.core);
            }
            KeyCode::Enter if event.modifiers.contains(REVERSE_MODIFIER) => {
                if let AppListPane::Buildings = self.list.pane
                    && let Some(i) = self.list.buildings.selected
                {
                    let Some(building) = Building::nth(i) else {
                        return Ok(());
                    };
                    if !self.core.sell_building(building) {
                        self.countdown
                            .error_tried_to_sell_unowned_building
                            .run(building);
                    }
                }
            }
            KeyCode::Enter => {
                if let AppListPane::Buildings = self.list.pane
                    && let Some(i) = self.list.buildings.selected
                {
                    let Some(building) = Building::nth(i) else {
                        return Ok(());
                    };
                    if !self.core.buy_building(building) {
                        self.countdown.error_insufficient_cookies.run();
                    }
                } else if let AppListPane::Upgrades = self.list.pane
                    && let Some(i) = self.list.upgrades.selected
                    && !self.core.buy_upgrade(i)
                {
                    self.countdown.error_insufficient_cookies.run();
                }
            }
            KeyCode::Esc => {
                if !matches!(self.modal, AppModalState::Closed) {
                    self.modal = AppModalState::Closed
                } else if self.debug.is_some() {
                    self.debug = None;
                } else {
                    self.quit().await?;
                }
            }
            KeyCode::Char(' ') => {
                self.core.click_cookie();
                self.countdown.just_pressed_cookie.run();
            }
            KeyCode::Char('q') => {
                self.quit().await?;
            }
            KeyCode::Char('i') => {
                self.modal.toggle();
            }
            KeyCode::Char('/') => {
                self.debug = self
                    .debug
                    .map(|v| v.next())
                    .or_else(|| Some(Default::default()));
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        term.draw(|frame| {
            let mut ui = crate::ui::UiApp {
                core: &self.core,
                list: &mut self.list,
                countdown: &self.countdown,
                modal: self.modal,
                debug: self.debug,
                debug_latest_key: self.debug_latest_key,
            };
            crate::ui::ui(&mut ui, frame);
        })
        .context("failed to draw app")?;
        Ok(())
    }

    async fn tick(&mut self) -> Result<()> {
        self.core.tick();
        self.countdown.tick();
        self.storage.tick(&self.core).await
    }

    async fn quit(&mut self) -> Result<()> {
        self.quit = true;
        self.storage.save(&self.core).await
    }
}

impl AppListState {
    fn up(&mut self) {
        self.state_mut().previous();
    }

    fn down(&mut self) {
        self.state_mut().next();
    }

    fn left(&mut self, core: &Core) {
        self.switch(core, AppListPane::prev);
    }

    fn right(&mut self, core: &Core) {
        self.switch(core, AppListPane::next);
    }

    fn switch(&mut self, core: &Core, change: fn(AppListPane) -> AppListPane) {
        let mut new_pane = change(self.pane);
        loop {
            if new_pane.available(core) {
                break;
            }
            new_pane = change(new_pane);
        }

        self.pane = new_pane;
        self.state_mut().select(Some(0));
    }

    pub fn selected(&self) -> Option<(AppListPane, usize)> {
        self.state().selected.map(|i| (self.pane, i))
    }

    pub fn state(&self) -> &ListState {
        match self.pane {
            AppListPane::Buildings => &self.buildings,
            AppListPane::Upgrades => &self.upgrades,
        }
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        match self.pane {
            AppListPane::Buildings => &mut self.buildings,
            AppListPane::Upgrades => &mut self.upgrades,
        }
    }

    pub fn state_matching_mut(&mut self, pane: AppListPane) -> Option<&mut ListState> {
        (self.pane == pane).then(|| self.state_mut())
    }
}

impl AppListPane {
    fn available(self, core: &Core) -> bool {
        match self {
            Self::Buildings => true,
            Self::Upgrades => !core.upgrades().is_empty(),
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::Buildings => Self::Upgrades,
            Self::Upgrades => Self::Buildings,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Buildings => Self::Upgrades,
            Self::Upgrades => Self::Buildings,
        }
    }
}

impl AppCountdownState {
    pub fn just_pressed_cookie(&self) -> bool {
        self.just_pressed_cookie.is_running()
    }

    pub fn error_insufficient_cookies(&self) -> bool {
        self.error_insufficient_cookies.is_running()
    }

    pub fn tick(&mut self) {
        self.just_pressed_cookie.tick();
        self.error_insufficient_cookies.tick();
    }
}

impl AppModalState {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Closed => Self::ListItem,
            _ => Self::Closed,
        }
    }
}

impl AppDebugView {
    fn next(self) -> Self {
        match self {
            Self::Cookies => Self::Buildings,
            Self::Buildings => Self::Upgrades,
            Self::Upgrades => Self::Achievements,
            Self::Achievements => Self::Milk,
            Self::Milk => Self::Ticker,
            Self::Ticker => Self::Keypress,
            Self::Keypress => Self::Cookies,
        }
    }
}
