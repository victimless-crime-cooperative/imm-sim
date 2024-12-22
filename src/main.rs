use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod camera;
pub mod debug_environment;
pub mod physics;
pub mod player;

fn main() {
    App::new()
        // Foreign Plgins
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            WorldInspectorPlugin::default(),
        ))
        // Plugins from this crate
        .add_plugins((
            camera::CameraPlugin,
            debug_environment::DebugEnvironmentPlugin,
            player::PlayerPlugin,
        ))
        .run();
}
