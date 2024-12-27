use bevy::prelude::*;

use crate::ServerState;

use self::handle_incoming::{handle_connection_events, handle_handshake_events};

pub mod handle_incoming;
pub mod tracking;

pub struct ServerConnectionsPlugin;

impl Plugin for ServerConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_connection_events, handle_handshake_events)
                .run_if(in_state(ServerState::Running)),
        );
    }
}
