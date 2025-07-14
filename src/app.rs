mod building;
mod cookies;
mod state;
mod ticker;

use self::{state::State, ticker::Ticker};
use crate::event::{Event, Events};
use anyhow::{Context, Result};
use ratatui::{DefaultTerminal, widgets::ListState};

#[derive(Debug)]
pub struct App {
    pub state: State,
    pub ticker: Ticker,
    pub building_list_state: ListState,
    events: Events,
    quit: bool,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let state = State::new();
        let ticker = Ticker::new(&state);
        Self {
            state,
            ticker,
            building_list_state: ListState::default(),
            events: Events::new(),
            quit: false,
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
                        self.building_list_state.select_previous();
                    }
                    KeyCode::Down => {
                        self.building_list_state.select_next();
                    }
                    KeyCode::Char('b') => {
                        self.state.buildings.buy(building::Building::Farm);
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
    }

    fn quit(&mut self) {
        self.quit = true;
    }
}
