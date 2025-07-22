mod debug;
mod list;
mod modal;

pub use self::{
    debug::{AppDebugState, AppDebugView},
    list::{AppListPane, AppListPointee, AppListState},
    modal::AppModalState,
};

use crate::{
    event::{Event, Events, REVERSE_MODIFIER},
    storage::Storage,
};
use anyhow::{Context, Result};
use cookie_clicker_tui_core::{Building, Core};
use cookie_clicker_tui_utils::countdown::{Countdown, CountdownOf};
use crossterm::event::KeyEvent;
use ratatui::DefaultTerminal;

pub struct App {
    storage: Storage,
    core: Core,
    list: AppListState,
    countdown: AppCountdownState,
    modal: AppModalState,
    debug: AppDebugState,
    events: Events,
    quit: bool,
}

pub struct AppCountdownState {
    just_pressed_cookie: Countdown<3>,
    error_insufficient_cookies: Countdown<10>,
    error_tried_to_sell_unowned_building: CountdownOf<Building, 10>,
}

impl App {
    pub fn new(storage: Storage, core: Core) -> Self {
        Self {
            storage,
            core,
            list: AppListState::default(),
            countdown: AppCountdownState {
                just_pressed_cookie: Countdown::new(),
                error_insufficient_cookies: Countdown::new(),
                error_tried_to_sell_unowned_building: CountdownOf::new(),
            },
            modal: AppModalState::default(),
            debug: AppDebugState::default(),
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

        self.debug.set_latest_key_event(event);

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
                match self.list.pointee(&self.core) {
                    Some((_, AppListPointee::Building(building))) => {
                        if !self.core.sell_building(building) {
                            self.countdown
                                .error_tried_to_sell_unowned_building
                                .run(building);
                        }
                    }
                    Some((_, AppListPointee::Upgrade(_))) => {
                        todo!()
                    }
                    None => {}
                }
            }

            KeyCode::Enter => match self.list.pointee(&self.core) {
                Some((_, AppListPointee::Building(building))) => {
                    if !self.core.buy_building(building) {
                        self.countdown.error_insufficient_cookies.run();
                    }
                }
                Some((i, AppListPointee::Upgrade(_))) => {
                    if !self.core.buy_upgrade(i) {
                        self.countdown.error_insufficient_cookies.run();
                    }
                }
                None => {}
            },
            KeyCode::Esc => {
                if self.modal.is_open() {
                    self.modal.close();
                } else if self.debug.is_open() {
                    self.debug.close();
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
                self.debug.advance();
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
                debug: &self.debug,
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
