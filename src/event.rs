use bevy_ecs::prelude::*;
use crossterm::event::KeyCode;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Event)]
pub struct KeyEvent(crossterm::event::KeyEvent);

impl KeyEvent {
    pub fn read() -> std::io::Result<Option<Self>> {
        Ok(crossterm::event::read()?.as_key_press_event().map(Self))
    }
}

pub fn handler(event: On<KeyEvent>) {
    match event.0.code {
        KeyCode::Char('q') => {
            QUITTING.store(true, Ordering::Relaxed);
        }
        _ => {}
    }
}

static QUITTING: AtomicBool = AtomicBool::new(false);

pub fn is_quitting() -> bool {
    QUITTING.load(Ordering::Relaxed)
}
