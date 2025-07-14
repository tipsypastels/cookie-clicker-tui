mod building;
mod cookies;
mod state;
mod ticker;
mod util;

pub use self::{
    building::{Building, Buildings},
    cookies::Cookies,
};

use self::{state::State, ticker::Ticker, util::countdown::Countdown};
use crate::event::{Event, Events};
use anyhow::{Context, Result};
use ratatui::DefaultTerminal;
use tui_widget_list::ListState;

#[derive(Debug)]
pub struct App {
    state: State,
    ticker: Ticker,
    list: AppList,
    just_pressed_cookie_countdown: Countdown<3>,
    error_insufficient_cookies_countdown: Countdown<10>,
    events: Events,
    quit: bool,
}

#[derive(Default, Debug)]
pub struct AppList {
    state: ListState,
    pane: ListStatePane,
}

#[derive(Debug)] // pre-borrows all of the disjoint fields so they can be accessed independantly
pub struct AppDeconstructedForRendering<'a> {
    pub buildings: &'a Buildings,
    pub cookies: &'a Cookies,
    pub ticker: Option<&'static str>,
    pub list: &'a mut AppList,
    _priv: (),
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum ListStatePane {
    #[default]
    Buildings,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let state = State::new();
        let ticker = Ticker::new(&state);
        Self {
            state,
            ticker,
            list: AppList::default(),
            just_pressed_cookie_countdown: Countdown::new(),
            error_insufficient_cookies_countdown: Countdown::new(),
            events: Events::new(),
            quit: false,
        }
    }

    pub fn buildings(&self) -> &Buildings {
        &self.state.buildings
    }

    pub fn cookies(&self) -> &Cookies {
        &self.state.cookies
    }

    pub fn ticker(&self) -> Option<&'static str> {
        self.ticker.text()
    }

    pub fn deconstruct_for_rendering(&mut self) -> AppDeconstructedForRendering {
        AppDeconstructedForRendering {
            buildings: &self.state.buildings,
            cookies: &self.state.cookies,
            ticker: self.ticker.text(),
            list: &mut self.list,
            _priv: (),
        }
    }

    pub fn list_state_for_pane(&mut self, pane: ListStatePane) -> Option<&mut ListState> {
        self.list.state_for_pane(pane)
    }

    pub fn just_pressed_cookie(&self) -> bool {
        self.just_pressed_cookie_countdown.is_running()
    }

    pub fn error_insufficient_cookies(&self) -> bool {
        self.error_insufficient_cookies_countdown.is_running()
    }

    pub async fn run(mut self, term: &mut DefaultTerminal) -> Result<()> {
        while !self.quit {
            self.draw(term)?;

            use crossterm::event::{Event::Key, KeyCode};

            match self.events.next().await? {
                Event::Tick => {
                    self.tick();
                }
                Event::Term(Key(event)) if event.is_press() => match event.code {
                    KeyCode::Up => {
                        self.list.state.previous();
                    }
                    KeyCode::Down => {
                        self.list.state.next();
                    }
                    #[allow(clippy::single_match)] // for now
                    KeyCode::Enter => match (self.list.state.selected, self.list.pane) {
                        (Some(i), ListStatePane::Buildings) => {
                            let Some(building) = Building::get(i) else {
                                continue;
                            };

                            let cost = self.state.buildings.cost(building);
                            if self.state.cookies < cost {
                                self.error_insufficient_cookies_countdown.run();
                                continue;
                            }

                            self.state.cookies.sub(cost);
                            self.state.buildings.buy(building);
                        }
                        _ => {}
                    },
                    KeyCode::Char(' ') => {
                        self.state.cookies.add(1.0);
                        self.just_pressed_cookie_countdown.run();
                    }
                    KeyCode::Char('q') => {
                        self.quit();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        term.draw(|f| {
            crate::ui::ui(self, f);
        })
        .context("failed to draw app")?;
        Ok(())
    }

    fn tick(&mut self) {
        self.state.cookies.tick(self.state.buildings.cps());
        self.ticker.tick(&self.state);
        self.error_insufficient_cookies_countdown.tick();
        self.just_pressed_cookie_countdown.tick();
    }

    fn quit(&mut self) {
        self.quit = true;
    }
}

impl AppList {
    pub fn state_for_pane(&mut self, pane: ListStatePane) -> Option<&mut ListState> {
        if self.pane == pane {
            Some(&mut self.state)
        } else {
            None
        }
    }
}
