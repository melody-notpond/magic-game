pub use bevy::prelude::*;
use voxel::VoxelPlugin;

pub mod client;
pub mod magic;
pub mod voxel;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(VoxelPlugin)
            .add_systems(Startup, (client::setup_camera, client::setup_scene))
            .add_systems(Update, (
                client::handle_input,
                client::handle_mouse,
                client::generate_chunk
            ))
        ;
    }
}

