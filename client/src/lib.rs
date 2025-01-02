use bevy::prelude::*;
use imm_sim_shared::ProtocolPlugin;

use self::{
    connect::FormConnectionPlugin, input::InputCollectionPlugin, physics::ClientPhysicsPlugin,
    player::ClientPlayerPlugin,
};

pub mod camera;
pub mod connect;
pub mod debug_environment;
pub mod input;
pub mod physics;
pub mod player;

pub struct ImmSimClientPlugin;

impl Plugin for ImmSimClientPlugin {
    fn build(&self, app: &mut App) {
        // Protocol
        app.add_plugins(ProtocolPlugin);
        // GUI to connect to a server.
        app.add_plugins(FormConnectionPlugin);
        // Collect inputs and commands
        app.add_plugins(InputCollectionPlugin);
        // State sync
        app.add_plugins((ClientPhysicsPlugin, ClientPlayerPlugin));
        // ClientSide Camera
        app.add_plugins(camera::CameraPlugin);
        // Simple geometry to test movement
        app.add_plugins(debug_environment::DebugEnvironmentPlugin);
    }
}
