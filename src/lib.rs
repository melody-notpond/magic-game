pub use bevy::prelude::*;
use client::NoiseChunkGen;
use voxel::VoxelPlugin;

pub mod client;
pub mod magic;
pub mod voxel;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(VoxelPlugin::new(NoiseChunkGen::default()))
            .add_systems(Startup, (client::setup_camera, client::setup_scene))
            .add_systems(Update, (
                client::handle_input,
                client::handle_mouse,
            ))
        ;
    }
}

