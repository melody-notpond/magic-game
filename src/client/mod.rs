use crate::*;

#[derive(Component)]
pub struct Player;

pub(crate) fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(5.0)).into(),
        material: materials.add(Color::rgb(1.0, 1.0, 0.0)),
        ..Default::default()
    }, Player));
}

pub(crate) fn move_player(
    mut q: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut trans = q.single_mut();
    const SPEED: f32 = 25.0;

    let mut add = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        add += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyS) {
        add += Vec3::NEG_Y;
    }
    if keys.pressed(KeyCode::KeyA) {
        add += Vec3::NEG_X;
    }
    if keys.pressed(KeyCode::KeyD) {
        add += Vec3::X;
    }

    trans.translation += add.normalize_or_zero() * SPEED * time.delta_seconds();
}
