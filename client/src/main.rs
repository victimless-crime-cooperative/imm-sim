use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::RepliconRenetPlugins;
use imm_sim::ImmSimClientPlugin;

fn main() {
    let mut app = App::new();

    // Default plugins, physics-related plugins, and netcode-related plugins in that order.
    app.add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((RepliconPlugins, RepliconRenetPlugins));

    // If running in a debug buld, include the [`WorldInspectorPlugin`].
    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::default());

    // Add the game logic and run.
    app.add_plugins(ImmSimClientPlugin).run();
}
