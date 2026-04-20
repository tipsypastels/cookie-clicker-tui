mod event;
mod render;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use std::time::{Duration, Instant};

const FPS: f64 = 30.0;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Startup;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct Update;

fn main() -> std::io::Result<()> {
    let mut startup = Schedule::new(Startup);
    let mut update = Schedule::new(Update);

    update.add_systems(render::render);

    let mut world = World::new();

    world.add_schedule(startup);
    world.add_schedule(update);

    world.init_resource::<render::Terminal>();
    world.add_observer(event::handler);

    world.flush();
    world.run_schedule(Startup);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs_f64(1.0 / FPS);

    loop {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !crossterm::event::poll(timeout)? {
            world.run_schedule(Update);
            last_tick = Instant::now();
        } else if let Some(event) = event::KeyEvent::read()? {
            world.trigger(event);
        }

        world.flush();

        if world.is_resource_added::<event::Quitting>() {
            render::Terminal::restore();
            break Ok(());
        }
    }
}
