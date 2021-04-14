use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

#[derive(Default)]
pub struct BoxConfig {
    pub dynamic: bool,
    pub position: Vec2,
    pub size: Option<Vec2>,
    pub material: Handle<ColorMaterial>,
}

pub fn spawn_box(commands: &mut Commands, config: BoxConfig) {
    let body = if config.dynamic {
        RigidBodyBuilder::new_dynamic()
    } else {
        RigidBodyBuilder::new_static()
    }.translation(config.position.x, config.position.y);

    let size = config.size.unwrap_or(Vec2::ONE);
    let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0);
    
    commands.spawn_bundle(SpriteBundle {
        material: config.material,
        sprite: Sprite::new(size),
        transform: Transform::from_xyz(config.position.x, config.position.y, 0.),
        ..Default::default()
    })
    .insert(body)
    .insert(collider);
}
