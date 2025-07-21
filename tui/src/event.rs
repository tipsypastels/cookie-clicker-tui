use anyhow::{Context, Result};
use cookie_clicker_tui_utils::frames::FPS;
use crossterm::event::{EventStream, KeyModifiers};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

// the vscode integrated terminal does not seem to support SHIFT,
// so use ALT while in development.
#[cfg(debug_assertions)]
pub const REVERSE_MODIFIER: KeyModifiers = KeyModifiers::ALT;
#[cfg(not(debug_assertions))]
pub const REVERSE_MODIFIER: KeyModifiers = KeyModifiers::SHIFT;

pub enum Event {
    Tick,
    Term(crossterm::event::Event),
}

pub struct Events {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let actor = Actor { tx };
        tokio::spawn(async move { actor.run().await });
        Self { rx }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx.recv().await.context("failed to receive event")
    }
}

struct Actor {
    tx: mpsc::UnboundedSender<Event>,
}

impl Actor {
    async fn run(self) -> Result<()> {
        let tick_rate = Duration::from_secs_f64(1.0 / FPS);
        let mut reader = EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);

        loop {
            tokio::select! {
                _ = self.tx.closed() => {
                    break;
                }
                _ = tick.tick() => {
                    _ = self.tx.send(Event::Tick);
                }
                Some(Ok(event)) = reader.next().fuse() => {
                    _ = self.tx.send(Event::Term(event));
                }
            }
        }

        Ok(())
    }
}
