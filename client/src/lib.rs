use bevy::prelude::*;
use imm_sim_shared::ProtocolPlugin;

use self::{
    connect::FormConnectionPlugin, input::InputCollectionPlugin, state_mirroring::MirrorStatePlugin,
};

pub mod connect;
pub mod input;
pub mod state_mirroring;

pub struct ImmSimClientPlugin;

impl Plugin for ImmSimClientPlugin {
    fn build(&self, app: &mut App) {
        // Protocol
        app.add_plugins(ProtocolPlugin);
        // GUI to connect to a server.
        app.add_plugins(FormConnectionPlugin);
        // Collect inputs and commands
        app.add_plugins(InputCollectionPlugin);
        // Mirror state from replicated components to regular components.
        app.add_plugins(MirrorStatePlugin);
    }
}
