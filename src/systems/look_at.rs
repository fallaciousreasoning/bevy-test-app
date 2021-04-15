use super::components::LookAt;
use bevy::math::Quat;
use bevy::prelude::{Entity, Query, QuerySet, Transform};
use bevy_rapier2d::physics::RigidBodyHandleComponent;

pub fn look_at(
    lookers: Query<(&LookAt, Entity)>,
    mut targets: QuerySet<(Query<&Transform>, Query<(&mut Transform, Option<&mut RigidBodyHandleComponent>)>)>,
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

    }
}
