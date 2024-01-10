use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..Default::default()
    });
}

pub fn move_camera(
    mut query: Query<&mut Transform, With<Camera2d>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_wheel: ResMut<Events<MouseWheel>>,
) {
    let mut transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += 10.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= 10.;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= 10.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 10.;
    }

    mouse_wheel.drain().for_each(|e| {
        transform.scale += -e.y * 0.01;
    });
}
