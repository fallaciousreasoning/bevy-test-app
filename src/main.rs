use bevy::prelude::*;
use bevy_rapier2d::{physics::RapierPhysicsPlugin, rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder}};

pub mod components;
pub mod systems;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(initialize_world.system())
        .add_startup_system(make_wall.system())
        .add_startup_system(make_player.system())
        .add_system(systems::mover.system())
        .add_system(systems::player_controller.system())
        .run();
}

fn initialize_world(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale = Vec3::new(1.0/64.0, 1.0/64.0, 1.0);
    commands.spawn_bundle(camera);
}

fn make_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(components::Velocity { x: 0.0, y: 0.0 })
        .insert(components::Character { speed: 10.0 });
}

fn make_wall(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let body = RigidBodyBuilder::new_dynamic().gravity_scale(0.0);
    let collider = ColliderBuilder::cuboid(0.5, 2.0);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0,1.0,1.0).into()),
        sprite: Sprite::new(Vec2::new(1.0,4.0)),
        transform: Transform::from_xyz(5.0, 0.0, 0.0),
        ..Default::default()
    }).insert(body).insert(collider);
}
