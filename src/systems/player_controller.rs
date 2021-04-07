use super::components::*;
use bevy::prelude::{Input, KeyCode, Query, Res};

pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Character)>,
) {
    for (mut velocity, character) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x = -character.speed;
        } else if keyboard_input.pressed(KeyCode::Right) {
            velocity.x = character.speed;
        } else {
            velocity.x = 0.0;
        }
    }
}
