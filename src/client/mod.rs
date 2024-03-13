use bevy::{window::CursorGrabMode, input::mouse::MouseMotion};

use crate::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct Paused(bool);

pub(crate) fn setup_scene(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    commands.init_resource::<Paused>();
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }, Player));
}

pub(crate) fn handle_mouse(
    paused: Res<Paused>,
    mut cursor_events: EventReader<MouseMotion>,
    mut q: Query<&mut Transform, With<Player>>,
) {
    if paused.0 {
        return;
    }

    let mut trans = q.single_mut();
    for event in cursor_events.read() {
        trans.rotate_axis(Vec3::Y, -event.delta.x / 200.0);
        let x = trans.local_x().into();
        trans.rotate_axis(x, -event.delta.y / 200.0);
    }
}

pub(crate) fn handle_input(
    mut paused: ResMut<Paused>,
    mut windows: Query<&mut Window>,
    mut q: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Escape) {
        paused.0 ^= true;
        window.cursor.visible = paused.0;
        window.cursor.grab_mode = if paused.0 {
            CursorGrabMode::None
        } else {
            CursorGrabMode::Locked
        };
    }

    if paused.0 {
        return;
    }

    let mut trans = q.single_mut();
    let mut forward: Vec3 = trans.forward().into();
    forward.y = 0.0;
    let left: Vec3 = trans.left().into();
    const SPEED: f32 = 15.0;

    let mut add = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        add += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        add += -forward;
    }
    if keys.pressed(KeyCode::KeyA) {
        add += left;
    }
    if keys.pressed(KeyCode::KeyD) {
        add += -left;
    }

    trans.translation += add.normalize_or_zero() * SPEED * time.delta_seconds();

    if keys.pressed(KeyCode::Space) {
        trans.translation += Vec3::Y * SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        trans.translation += Vec3::NEG_Y * SPEED * time.delta_seconds();
    }
}
