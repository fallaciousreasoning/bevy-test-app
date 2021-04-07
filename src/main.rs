use bevy::prelude::*;

pub mod components;
pub mod systems;

fn main() {
    App::build()
        .add_system(systems::mover.system())
        .run();
}
