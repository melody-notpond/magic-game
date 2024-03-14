use bevy::{window::CursorGrabMode, input::mouse::MouseMotion};
use noise::{NoiseFn, Perlin};

use crate::voxel::{Voxels, CHUNK_DIM, CHUNK_SIZE_I32, VoxelConfigEntry, VOXEL_SIZE};
use crate::voxel::components::{GenerateChunk, ConstructChunkMesh, ChunkLoader};
use crate::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct Paused(bool);

#[derive(Resource, Default)]
pub struct Noise(Perlin);

pub(crate) fn setup_scene(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut voxels: ResMut<Voxels>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    voxels.add_voxel("solid", VoxelConfigEntry {
        debug_name: "solid".to_owned(),
        render: true,
        solid: true,
        color: Color::rgb_u8(255, 255, 0),
    });
    commands.init_resource::<Paused>();
    commands.init_resource::<Noise>();
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb_u8(255, 255, 200),
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(1.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }, Player, ChunkLoader {
        x_radius: 4,
        y_radius: 0,
        z_radius: 4,
    }));
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

pub(crate) fn generate_chunk(
    mut voxels: ResMut<Voxels>,
    mut rx: EventReader<GenerateChunk>,
    mut tx: EventWriter<ConstructChunkMesh>,
    noise: Res<Noise>,
) {
    for &GenerateChunk { x, y, z } in rx.read() {
        let (chunk_x, chunk_y, chunk_z) = (x, y, z);
        let solid = voxels.id_from_name("solid").unwrap();
        for x in 0..CHUNK_SIZE_I32 {
            for y in 0..5 {
                for z in 0..CHUNK_SIZE_I32 {
                    if noise.0.get([
                        (x as f32 * VOXEL_SIZE + chunk_x as f32 * CHUNK_DIM) as f64 * 0.07,
                        (y as f32 * VOXEL_SIZE + chunk_y as f32 * CHUNK_DIM) as f64 * 0.07,
                        (z as f32 * VOXEL_SIZE + chunk_z as f32 * CHUNK_DIM) as f64 * 0.07,
                    ]) > 0.0 {
                        voxels.set_block(
                            chunk_x * CHUNK_SIZE_I32 + x,
                            chunk_y * CHUNK_SIZE_I32 + y,
                            chunk_z * CHUNK_SIZE_I32 + z,
                            solid
                        );
                    }
                }
            }
        }

        tx.send(ConstructChunkMesh { x, y, z });
    }
}
