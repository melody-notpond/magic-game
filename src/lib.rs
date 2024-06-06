#[allow(ambiguous_glob_reexports)] // uncomfy with this but ignore
pub use bevy::prelude::*;
pub use bevy_rapier3d::prelude::*;
pub use lightyear::prelude::*;

use client_plugin::NoiseChunkGen;
use voxel::VoxelPlugin;

pub mod client_plugin;
pub mod magic;
pub mod net;
pub mod voxel;
pub mod version;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(VoxelPlugin::new(NoiseChunkGen::default()))
            .add_systems(Startup, (
                client_plugin::setup_player,
                client_plugin::setup_scene))
            .add_systems(Update, (
                client_plugin::handle_input,
                client_plugin::handle_mouse))
        ;
    }
}

