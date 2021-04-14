use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::{
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};
use spawns::{BoxConfig, spawn_box};
pub mod components;
pub mod systems;
pub mod spawns;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(initialize_world.system())
        .add_startup_system(make_walls.system())
        .add_startup_system(make_player.system())
        .add_system(systems::mover.system())
        .add_system(systems::player_controller.system())
        .run();
}

fn initialize_world(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.scale = 1.0;
    rapier_config.gravity = Vector2::zeros();
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = Vec3::new(1.0 / 64.0, 1.0 / 64.0, 1.0);
    commands.spawn_bundle(camera);
    
    spawn_box(&mut commands, BoxConfig {
        dynamic: true,
        ..Default::default()
    });
}

fn make_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::cuboid(0.5, 0.5);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(components::Velocity { x: 0.0, y: 0.0 })
        .insert(components::Character { speed: 10.0 })
        .insert(body)
        .insert(collider);
}

fn make_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::WHITE.into());
    spawn_box(&mut commands, BoxConfig {
        dynamic: false,
        material: wall_material.clone(),
        position: Vec2::new(-10., 0.),
        size: Some(Vec2::new(1., 1000.))
    });

    spawn_box(&mut commands, BoxConfig {
        dynamic: false,
        material: wall_material.clone(),
        position: Vec2::new(10., 0.),
        size: Some(Vec2::new(1., 1000.))
    });

    spawn_box(&mut commands, BoxConfig {
        dynamic: false,
        material: wall_material.clone(),
        position: Vec2::new(0., -6.),
        size: Some(Vec2::new(1000., 1.))
    });

    spawn_box(&mut commands, BoxConfig {
        dynamic: false,
        material: wall_material.clone(),
        position: Vec2::new(0., 6.),
        size: Some(Vec2::new(1000., 1.))
    });
}
