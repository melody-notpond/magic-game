use bevy::{window::CursorGrabMode, input::mouse::MouseMotion};
use noise::{NoiseFn, Perlin};

use crate::voxel::{VoxelRes, CHUNK_DIM, CHUNK_SIZE_I32, VoxelConfigEntry, VOXEL_SIZE};
use crate::voxel::components::ChunkLoader;
use crate::*;

use self::voxel::{ChunkGenerator, VoxelId, Voxels, CHUNK_SIZE_CB};

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct Paused(bool);

pub(crate) fn setup_scene(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    voxels: Res<VoxelRes>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    let Ok(mut voxels) = voxels.write()
    else {
        return;
    };

    voxels.add_voxel("solid", VoxelConfigEntry {
        debug_name: "solid".to_owned(),
        render: true,
        solid: true,
        color: Color::rgb_u8(255, 255, 0),
    });
    commands.init_resource::<Paused>();
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

pub(crate) fn setup_player(mut commands: Commands) {
    commands.spawn((TransformBundle {
            local: Transform::from_xyz(4.0, 8.0, 4.0)
                .looking_to(Vec3::NEG_Z, Vec3::Y),
            ..Default::default()
        }, Player, ChunkLoader {
            x_radius: 10,
            y_radius: 0,
            z_radius: 10
        }, RigidBody::KinematicPositionBased,
        Collider::capsule_y(1.7, 0.4),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.51),
                min_width: CharacterLength::Absolute(0.49),
                include_dynamic_bodies: false,
            }),
            snap_to_ground: Some(CharacterLength::Absolute(0.51)),
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        }))
    .with_children(|cs| {
        cs.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.6, 0.0)
                .looking_to(Vec3::NEG_Z, Vec3::Y),
            ..Default::default()
        });
    });
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
    mut q: Query<(&mut KinematicCharacterController, &Transform),
        With<Player>>,
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

    let (mut cont, trans) = q.single_mut();
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

    add = add.normalize_or_zero() * SPEED * time.delta_seconds();

    if keys.pressed(KeyCode::Space) {
        add += Vec3::Y * SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        add += Vec3::NEG_Y * SPEED * time.delta_seconds();
    }

    cont.translation = Some(add);
}

#[derive(Default)]
pub(crate) struct NoiseChunkGen {
    noise: Perlin,
}

impl ChunkGenerator for NoiseChunkGen {
    fn generate(&mut self, chunk_x: i32, chunk_y: i32, chunk_z: i32,
        voxels: &Voxels,
        chunk_voxels: &mut [VoxelId; CHUNK_SIZE_CB]) -> bool {
        let solid = voxels.id_from_name("solid").unwrap();
        for x in 0..CHUNK_SIZE_I32 {
            for y in 0..5 {
                for z in 0..CHUNK_SIZE_I32 {
                    if self.noise.get([
                        (x as f32 * VOXEL_SIZE + chunk_x as f32 * CHUNK_DIM)
                            as f64 * 0.07,
                        (y as f32 * VOXEL_SIZE + chunk_y as f32 * CHUNK_DIM)
                            as f64 * 0.07,
                        (z as f32 * VOXEL_SIZE + chunk_z as f32 * CHUNK_DIM)
                            as f64 * 0.07,
                    ]) > 0.0 {
                        voxel::set_chunk_voxel(
                            chunk_voxels,
                            x,
                            y,
                            z,
                            solid
                        );
                    }
                }
            }
        }

        true
    }
}
