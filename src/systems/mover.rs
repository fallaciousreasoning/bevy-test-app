use bevy::prelude::{Query, Transform, Res, Time};
use super::components::Velocity;

pub fn mover(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
       let translation = &mut transform.translation;
        translation.x += velocity.x * time.delta_seconds();
        translation.y += velocity.y * time.delta_seconds();
    }
}