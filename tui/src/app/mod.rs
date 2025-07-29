mod bakery;
mod debug;
mod interface;
mod list;
mod modal;
mod news;
mod tick;

pub use self::{
    bakery::AppBakery,
    debug::{AppDebugState, AppDebugView},
    interface::{AppFlash, AppInterfaceState},
    list::{AppListPane, AppListPointee, AppListState},
    modal::AppModalState,
    news::AppNewsState,
    tick::AppTickState,
};

use crate::{
    audio::Audio,
    event::{Event, Events},
    save::Save,
};
use anyhow::{Context, Result};
use cookie_clicker_tui_core::Core;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::DefaultTerminal;

pub struct App {
    save: Save,
    core: Core,
    audio: Audio,
    tick: AppTickState,
    list: AppListState,
    modal: AppModalState,
    iface: AppInterfaceState,
    debug: AppDebugState,
    news: AppNewsState,
    bakery: AppBakery,
    events: Events,
    quit: bool,
}

impl App {
    pub fn new(save: Save, core: Core, name: Option<Box<str>>, audio: Audio) -> Self {
        let news = AppNewsState::new(&core);
        Self {
            save,
            core,
            audio,
            tick: AppTickState::default(),
            list: AppListState::default(),
            modal: AppModalState::default(),
            iface: AppInterfaceState::default(),
            debug: AppDebugState::default(),
            news,
            bakery: AppBakery::new(name),
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
                    self.debug.set_latest_key_event(event);

                    match self.modal {
                        AppModalState::RenamingBakery(_) => {
                            self.handle_renaming_bakery_key_event(event);
                        }
                        AppModalState::Wrinklers { .. } => {
                            self.handle_wrinklers_key_event(event);
                        }
                        _ => {
                            self.handle_key_event(event).await?;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_renaming_bakery_key_event(&mut self, event: KeyEvent) {
        // can't pass this as a parameter without cloning since we're borrowing from self
        let name = match &mut self.modal {
            AppModalState::RenamingBakery(name) => name,
            _ => unreachable!(),
        };

        match event.code {
            KeyCode::Enter => {
                self.bakery.set_name(&**name);
                self.modal.close();
            }
            KeyCode::Esc => {
                self.modal.close();
            }
            KeyCode::Backspace => {
                name.pop();
            }
            KeyCode::Char(char) => {
                name.push(char);
            }
            _ => {}
        }
    }

    fn handle_wrinklers_key_event(&mut self, event: KeyEvent) {
        let state = match &mut self.modal {
            AppModalState::Wrinklers { state } => state,
            _ => unreachable!(),
        };

        match event.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('w') => {
                self.modal.close();
            }
            KeyCode::Up => {
                state.previous();
            }
            KeyCode::Down => {
                state.next();
            }
            _ => {}
        }
    }

    async fn handle_key_event(&mut self, event: KeyEvent) -> Result<()> {
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
                    Some((_, AppListPointee::Upgrade(upgrade))) => {
                        if !self.core.buy_upgrade(upgrade) {
                            self.iface.add_flash(AppFlash::CantAffordUpgrade(upgrade));
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
            KeyCode::Char(ch @ '1'..='9') => {
                if self.core.click_golden_cookie(ch) {
                    self.audio.golden_cookie_click();
                }
            }
            KeyCode::Char('q') => {
                self.quit().await?;
            }
            KeyCode::Char('i') => {
                self.modal.toggle_list_item();
            }
            KeyCode::Char('r') => {
                self.modal.set_renaming_bakery();
            }
            KeyCode::Char('s') => {
                if self.list.is_pane_highlighted(AppListPane::Buildings) {
                    self.iface.toggle_sell_mode();
                }
            }
            KeyCode::Char('w') => {
                if !self.core.grandmapocalypse().wrinklers().is_empty() {
                    self.modal.set_wrinklers();
                }
            }
            KeyCode::Char('.') => {
                self.debug.backward();
            }
            KeyCode::Char('/') => {
                self.debug.forward();
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        term.draw(|frame| {
            let mut ui = crate::ui::UiApp {
                save: &self.save,
                core: &self.core,
                tick: &self.tick,
                list: &mut self.list,
                iface: &self.iface,
                modal: &mut self.modal,
                news: &mut self.news,
                debug: &self.debug,
                bakery: &self.bakery,
            };
            crate::ui::ui(&mut ui, frame);
        })
        .context("failed to draw app")?;
        Ok(())
    }

    async fn tick(&mut self) -> Result<()> {
        self.core.tick();
        self.iface.tick();
        self.news.tick(&self.core);
        self.save.tick(&self.core, self.bakery.name()).await?;

        if self.core.sugar_lumps().just_unlocked() {
            self.iface.add_flash(AppFlash::SugarLumpsUnlocked);
        }

        if self.core.research().just_completed() {
            self.iface.add_flash(AppFlash::ResearchCompleted);
        }

        if self.save.notify_just_saved() {
            self.iface.add_flash(AppFlash::Saved);
        }

        if self.save.notify_swallowed_parse_error() && self.tick.is_very_first() {
            self.iface.add_flash(AppFlash::WontSaveOverParseError);
        }

        self.tick.tick();

        Ok(())
    }

    async fn quit(&mut self) -> Result<()> {
        self.quit = true;
        self.save.save(&self.core, self.bakery.name()).await
    }
}
