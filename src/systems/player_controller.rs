use bevy::prelude::{Query};
use super::components::*;

pub fn player_controller(mut query: Query<(&mut Velocity, &Character)>) {
    for (mut velocity, character) in query.iter_mut() {
        velocity.x = -character.speed;
    }
}