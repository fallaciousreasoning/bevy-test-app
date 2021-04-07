use bevy::prelude::*;

pub mod components;
pub mod systems;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initialize_world.system())
        .add_system(systems::mover.system())
        .run();
}

fn initialize_world(mut commands: Commands) {
    commands
        .spawn()
        .insert(components::Position { x: 6.0, y: 7.0 });
}
