pub use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use client::move_player;

pub mod magic;
pub mod client;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, client::setup_player))
            .add_systems(Update, move_player);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        // transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..Default::default()
    });
}
