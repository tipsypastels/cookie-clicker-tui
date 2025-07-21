use crate::{
    event::{Event, Events},
    storage::Storage,
};
use anyhow::{Context, Result};
use cookie_clicker_tui_core::{Building, Core};
use cookie_clicker_tui_utils::countdown::Countdown;
use ratatui::DefaultTerminal;
use tui_widget_list::ListState;

#[derive(Debug)]
pub struct App {
    storage: Storage,
    core: Core,
    list: AppListState,
    countdown: AppCountdownState,
    modal: AppModalState,
    events: Events,
    quit: bool,
}

#[derive(Debug)]
pub struct AppListState {
    state: ListState,
    pane: AppListPane,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum AppListPane {
    #[default]
    Buildings,
    Upgrades,
}

#[derive(Debug)]
pub struct AppCountdownState {
    just_pressed_cookie: Countdown<3>,
    error_insufficient_cookies: Countdown<10>,
}

#[derive(Default, Debug)]
pub enum AppModalState {
    Debug(String),
    ListItem,
    #[default]
    Closed,
}

impl App {
    pub fn new(storage: Storage, core: Core) -> Self {
        Self {
            storage,
            core,
            list: AppListState {
                state: ListState::default(),
                pane: AppListPane::default(),
            },
            countdown: AppCountdownState {
                just_pressed_cookie: Countdown::new(),
                error_insufficient_cookies: Countdown::new(),
            },
            modal: AppModalState::default(),
            events: Events::new(),
            quit: false,
        }
    }

    pub async fn run(mut self, term: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            use crossterm::event::{Event::Key, KeyCode};

            self.draw(term)?;

            match self.events.next().await? {
                Event::Tick => {
                    self.tick().await?;
                }
                Event::Term(Key(event)) if event.is_press() => match event.code {
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
                    KeyCode::Enter => match (self.list.state.selected, self.list.pane) {
                        (Some(i), AppListPane::Buildings) => {
                            let Some(building) = Building::nth(i) else {
                                continue;
                            };
                            if !self.core.buy_building(building) {
                                self.countdown.error_insufficient_cookies.run();
                            }
                        }
                        (Some(i), AppListPane::Upgrades) => {
                            if !self.core.buy_upgrade(i) {
                                self.countdown.error_insufficient_cookies.run();
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Esc => {
                        if !matches!(self.modal, AppModalState::Closed) {
                            self.modal = AppModalState::Closed
                        } else {
                            self.quit().await?;
                        }
                    }
                    KeyCode::Char(' ') => {
                        self.core.give_cookies(1.0);
                        self.countdown.just_pressed_cookie.run();
                    }
                    KeyCode::Char('q') => {
                        self.quit().await?;
                    }
                    KeyCode::Char('i') => {
                        self.modal.toggle();
                    }
                    KeyCode::Char('/') => {
                        self.modal.debug(format!("{:?}", self.core));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        term.draw(|frame| {
            let mut ui = crate::ui::UiApp {
                core: &self.core,
                list: &mut self.list,
                countdown: &self.countdown,
                modal: &self.modal,
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
        self.state.previous();
    }

    fn down(&mut self) {
        self.state.next();
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

        if let Some(cur) = self.state.selected {
            let max = new_pane.max(core);
            self.state.select(Some(std::cmp::min(cur, max)));
        }

        self.pane = new_pane;
    }

    pub fn selected(&self) -> Option<(AppListPane, usize)> {
        self.state.selected.map(|i| (self.pane, i))
    }

    pub fn pane(&mut self, pane: AppListPane) -> Option<&mut ListState> {
        (self.pane == pane).then_some(&mut self.state)
    }
}

impl AppListPane {
    fn available(self, core: &Core) -> bool {
        match self {
            Self::Buildings => true,
            Self::Upgrades => !core.upgrades().is_empty(),
        }
    }

    fn max(self, core: &Core) -> usize {
        match self {
            Self::Buildings => Building::VARIANT_COUNT - 1,
            Self::Upgrades => core.upgrades().len() - 1,
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

    fn debug(&mut self, message: String) {
        *self = Self::Debug(message)
    }
}
