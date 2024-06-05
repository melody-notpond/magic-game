#[allow(ambiguous_glob_reexports)] // uncomfy with this but ignore
pub use bevy::prelude::*;
pub use bevy_rapier3d::prelude::*;
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
            .add_systems(Startup, (client::setup_player, client::setup_scene))
            .add_systems(Update, (
                client::handle_input,
                client::handle_mouse,
            ))
        ;
    }
}

