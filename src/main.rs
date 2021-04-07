use bevy::prelude::*;

pub mod components;
mod systems;

fn main() {
    App::build()
        .add_system(systems::mover.system())
        .run();
}
