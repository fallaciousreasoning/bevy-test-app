use bevy::{prelude::{Query, Transform, Res}, window::Windows};

use crate::components::OnMouse;

pub fn on_mouse(windows: Res<Windows>, mut query: Query<(&OnMouse, &mut Transform)>) {
    let mouse_position = if let Some(cursor_position) = windows.get_primary().and_then(|window| window.cursor_position()) {
        cursor_position
    } else {
        return;
    } / 64.;

    for (_mouse, mut transform) in query.iter_mut() {
        transform.translation.x = mouse_position.x;
        transform.translation.y = mouse_position.y;
    }
}