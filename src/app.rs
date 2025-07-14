mod building;
mod cookies;
mod state;
mod ticker;

use self::{building::Building, state::State, ticker::Ticker};
use crate::event::{Event, Events};
use anyhow::{Context, Result};
use ratatui::{DefaultTerminal, widgets::ListState};

#[derive(Debug)]
pub struct App {
    pub state: State,
    pub ticker: Ticker,
    pub list_state: ListState,
    pub list_state_pane: ListStatePane,
    events: Events,
    quit: bool,
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
            list_state: ListState::default(),
            list_state_pane: ListStatePane::default(),
            events: Events::new(),
            quit: false,
        }
    }

    pub fn list_state_for_pane(&mut self, pane: ListStatePane) -> Option<&mut ListState> {
        if self.list_state_pane == pane {
            Some(&mut self.list_state)
        } else {
            None
        }
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
                        self.list_state.select_previous();
                    }
                    KeyCode::Down => {
                        self.list_state.select_next();
                    }
                    #[allow(clippy::single_match)] // for now
                    KeyCode::Enter => match (self.list_state.selected(), self.list_state_pane) {
                        (Some(i), ListStatePane::Buildings) => {
                            let Some(building) = Building::index(i) else {
                                continue;
                            };
                            self.state.buildings.buy(building);
                        }
                        _ => {}
                    },
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
    }

    fn quit(&mut self) {
        self.quit = true;
    }
}
