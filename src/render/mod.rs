use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct Terminal(ratatui::DefaultTerminal);

impl Default for Terminal {
    fn default() -> Self {
        Self(ratatui::init())
    }
}

impl Terminal {
    pub fn restore() {
        ratatui::restore();
    }
}

pub fn render() {
    println!("Rendering.");
}
