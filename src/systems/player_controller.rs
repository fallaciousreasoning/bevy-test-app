use super::components::*;
use bevy::prelude::{Input, KeyCode, Query, Res, ResMut};
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::na::Vector2;

pub fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    query: Query<(&RigidBodyHandleComponent, &Character)>,
) {
    for (handle, character) in query.iter() {
        let x_axis = -(keyboard_input.pressed(KeyCode::A) as i8)
            + (keyboard_input.pressed(KeyCode::D) as i8);
        let y_axis = -(keyboard_input.pressed(KeyCode::S) as i8)
            + (keyboard_input.pressed(KeyCode::W) as i8);

        if x_axis == 0 && y_axis == 0 {
            continue;
        }
        let direction = Vector2::new(x_axis as f32, y_axis as f32).normalize() * character.speed;
        if let Some(body) = rigid_bodies.get_mut(handle.handle()) {
            body.set_linvel(direction, true);
        }
    }
}
