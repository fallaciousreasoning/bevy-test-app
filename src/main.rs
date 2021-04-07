use bevy::prelude::*;

mod systems;

fn main() {
    App::build()
        .add_system(systems::mover.system())
        .run();
}
