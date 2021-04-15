use super::components::LookAt;
use bevy::math::Quat;
use bevy::prelude::{Entity, Query, QuerySet, ResMut, Transform};
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::na::{Unit};

pub fn look_at(
    lookers: Query<(&LookAt, Entity)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut targets: QuerySet<(
        Query<&Transform>,
        Query<(&mut Transform, Option<&RigidBodyHandleComponent>)>,
    )>,
) {
    for (looker, entity) in lookers.iter() {
        let target = match targets.q0().get(looker.0) {
            Ok(t) => t,
            Err(_) => continue,
        }
        .translation;

        let result = match targets.q1_mut().get_mut(entity) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let mut transform = result.0;
        let dir = target - transform.translation;

        let rotation = dir.y.atan2(dir.x);
        transform.rotation = Quat::from_rotation_z(rotation);

        let body_handle = match result.1 {
            Some(b) => b,
            None => continue,
        };

        let body = match rigid_bodies.get_mut(body_handle.handle()) {
            Some(b) => b,
            None => continue,
        };

        let mut p = body.position().clone();
        p.rotation = Unit::from_angle(rotation);
        body.set_position(p, true);
    }
}
