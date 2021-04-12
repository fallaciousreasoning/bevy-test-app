use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::{
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};
pub mod components;
pub mod systems;

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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    rapier_config.scale = 1.0;
    rapier_config.gravity = Vector2::zeros();
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = Vec3::new(1.0 / 64.0, 1.0 / 64.0, 1.0);
    commands.spawn_bundle(camera);
    
    let width = 1.0;
    let height = 4.0;
    let x = 4.0;
    let y = 4.0;
    let dynamic = true;
    let body = if dynamic {
        RigidBodyBuilder::new_dynamic()
    } else {
        RigidBodyBuilder::new_static()
    }.translation(x, y);
    let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert(body)
        .insert(collider);
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

fn make_box(
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    dynamic: bool,
    mut commands: Commands,
    materials: &ResMut<Assets<ColorMaterial>>,
) {
    let mut materials = materials;
    let body = if dynamic {
        RigidBodyBuilder::new_dynamic()
    } else {
        RigidBodyBuilder::new_static()
    }.translation(x, y);
    let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert(body)
        .insert(collider);
}

fn make_walls(commands: Commands, materials: ResMut<Assets<ColorMaterial>>) {
    make_box(1.0, 1000.0, -10.0, 0.0, false, commands, &materials);
}
