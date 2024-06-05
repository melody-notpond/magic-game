use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
// use bevy_rapier3d::render::{
//     DebugRenderMode, DebugRenderStyle, RapierDebugRenderPlugin};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use magic_game::*;

mod fps;
use fps::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin {
        //     enabled: true,
        //     style: DebugRenderStyle::default(),
        //     mode: DebugRenderMode::all(),
        // })
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, fps_text_update_system)
        .run();
}
