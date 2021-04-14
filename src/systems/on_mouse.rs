use bevy::{prelude::{Query, Res, Transform, Vec2, With}, window::Windows};

use crate::components::OnMouse;

pub fn on_mouse(windows: Res<Windows>, mut query: Query<&mut Transform, With<OnMouse>>) {
    let primary_window = windows.get_primary();
    let mouse_position = if let Some(cursor_position) = primary_window.and_then(|window| window.cursor_position()) {
        cursor_position
    } else {
        return;
    } / 64. - primary_window.and_then(|w| Some(Vec2::new(w.width(), w.height()) / 2. / 64.)).unwrap();

    for mut transform in query.iter_mut() {
        transform.translation.x = mouse_position.x;
        transform.translation.y = mouse_position.y;
    }
}