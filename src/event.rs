use bevy_ecs::prelude::*;
use crossterm::event::KeyCode;

#[derive(Event)]
pub struct KeyEvent(crossterm::event::KeyEvent);

impl KeyEvent {
    pub fn read() -> std::io::Result<Option<Self>> {
        Ok(crossterm::event::read()?.as_key_press_event().map(Self))
    }
}

#[derive(Resource)]
pub struct Quitting;

pub fn handler(event: On<KeyEvent>, mut commands: Commands) {
    match event.0.code {
        KeyCode::Char('q') => {
            commands.insert_resource(Quitting);
        }
        _ => {}
    }
}
