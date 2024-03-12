pub use bevy::prelude::*;
use client::Paused;

pub mod magic;
pub mod client;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (client::setup_camera, client::setup_scene))
            .add_systems(Update, (client::handle_input, client::handle_mouse))
        ;
    }
}

