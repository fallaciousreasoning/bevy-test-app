use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

pub mod components;
pub mod spawns;
pub mod systems;
pub mod world_info;

use components::OnMouse;
use spawns::{spawn_box, BoxConfig};
use world_info::WorldInfo;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(initialize_world.system())
        .add_startup_system(make_walls.system())
        .add_system(systems::mover.system())
        .add_system(systems::player_controller.system())
        .add_system(systems::on_mouse.system())
        .add_system(systems::look_at.system())
        .run();
}

fn initialize_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.scale = 1.0;
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = Vec3::new(1.0 / 64.0, 1.0 / 64.0, 1.0);
    let camera_id = commands.spawn_bundle(camera).id();

    for i in 0..10 {
        spawn_box(
            &mut commands,
            BoxConfig {
                position: Vec2::new(2., -i as f32),
                dynamic: true,
                ..Default::default()
            },
        );
    }

    let cursor_id = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(1., 1.)),
            ..Default::default()
        })
        .insert(OnMouse)
        .id();

    let body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::cuboid(0.5, 0.5);
    let player_id = commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(components::Velocity { x: 0.0, y: 0.0 })
        .insert(components::Character { speed: 10.0 })
        .insert(body)
        .insert(collider)
        .insert(components::LookAt(cursor_id))
        .id();

    commands.insert_resource(WorldInfo {
        camera: cursor_id,
        cursor: camera_id,
        player: player_id,
    });
}

fn make_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let wall_material = materials.add(Color::WHITE.into());
    spawn_box(
        &mut commands,
        BoxConfig {
            dynamic: false,
            material: wall_material.clone(),
            position: Vec2::new(-10., 0.),
            size: Some(Vec2::new(1., 1000.)),
        },
    );

    spawn_box(
        &mut commands,
        BoxConfig {
            dynamic: false,
            material: wall_material.clone(),
            position: Vec2::new(10., 0.),
            size: Some(Vec2::new(1., 1000.)),
        },
    );

    spawn_box(
        &mut commands,
        BoxConfig {
            dynamic: false,
            material: wall_material.clone(),
            position: Vec2::new(0., -6.),
            size: Some(Vec2::new(1000., 1.)),
        },
    );

    spawn_box(
        &mut commands,
        BoxConfig {
            dynamic: false,
            material: wall_material.clone(),
            position: Vec2::new(0., 6.),
            size: Some(Vec2::new(1000., 1.)),
        },
    );
}
