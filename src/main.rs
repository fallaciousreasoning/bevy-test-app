use bevy::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;

pub mod components;
pub mod systems;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(initialize_world.system())
        .add_system(systems::mover.system())
        .add_system(systems::player_controller.system())
        .run();
}

fn initialize_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(64.0, 64.0)),
            ..Default::default()
        })
        .insert(components::Velocity { x: 0.0, y: 0.0 })
        .insert(components::Character { speed: 200.0 });
}
