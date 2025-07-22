mod debug;
mod interface;
mod list;
mod modal;

pub use self::{
    debug::{AppDebugState, AppDebugView},
    interface::{AppFlash, AppInterfaceState},
    list::{AppListPane, AppListPointee, AppListState},
    modal::AppModalState,
};

use crate::{
    event::{Event, Events},
    storage::Storage,
};
use anyhow::{Context, Result};
use cookie_clicker_tui_core::Core;
use crossterm::event::KeyEvent;
use ratatui::DefaultTerminal;

pub struct App {
    storage: Storage,
    core: Core,
    list: AppListState,
    modal: AppModalState,
    iface: AppInterfaceState,
    debug: AppDebugState,
    events: Events,
    quit: bool,
}

impl App {
    pub fn new(storage: Storage, core: Core) -> Self {
        Self {
            storage,
            core,
            list: AppListState::default(),
            modal: AppModalState::default(),
            iface: AppInterfaceState::default(),
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
            KeyCode::Enter =>
            {
                #[allow(clippy::collapsible_else_if)]
                match self.list.pointee(&self.core) {
                    Some((_, AppListPointee::Building(building))) => {
                        if self.iface.sell_mode() {
                            if !self.core.sell_building(building) {
                                self.iface
                                    .add_flash(AppFlash::CantSellUnownedBuilding(building));
                            }
                        } else {
                            if !self.core.buy_building(building) {
                                self.iface.add_flash(AppFlash::CantAffordBuilding(building));
                            }
                        }
                    }
                    Some((i, AppListPointee::Upgrade(_))) => {
                        if !self.core.buy_upgrade(i) {
                            self.iface.add_flash(AppFlash::CantAffordUpgrade(i));
                        }
                    }
                    None => {}
                }
            }
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
                self.iface.set_pressed_cookie();
            }
            KeyCode::Char('q') => {
                self.quit().await?;
            }
            KeyCode::Char('i') => {
                self.modal.toggle();
            }
            KeyCode::Char('s') => {
                if self.list.is_pane_highlighted(AppListPane::Buildings) {
                    self.iface.toggle_sell_mode();
                }
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
                iface: &self.iface,
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
        self.iface.tick();
        self.storage.tick(&self.core).await
    }

    async fn quit(&mut self) -> Result<()> {
        self.quit = true;
        self.storage.save(&self.core).await
    }
}
