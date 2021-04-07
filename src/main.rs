use bevy::prelude::*;

pub mod components;
pub mod systems;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(initialize_world.system())
        .add_system(systems::mover.system())
        .add_system(systems::player_controller.system())
        .run();
}

fn initialize_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("square.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .insert(components::Velocity { x: 0.0, y: 0.0 })
        .insert(components::Character { speed: 200.0 });
}
