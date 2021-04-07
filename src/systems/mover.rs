use bevy::prelude::{Query, Transform};
use super::components::Velocity;

pub fn mover(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
       let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;
        println!("{:?}", velocity);
    }
}