use bevy::prelude::*;
use bevy_replicon::prelude::*;

use self::{
    handshake::{C2SHandshakeStart, S2CHandshakeResult},
    ownership::OwnedByClient,
    physics::components::transform::ReplicatedTransform,
    player::{
        components::{PlayerAvatarColor, PlayerDisplayName},
        messages::{
            client_input::{C2SCommand, C2SInputEvent},
            server_commands::S2CSpawnPlayerCommand,
        },
    },
};

pub mod handshake;
pub mod ownership;
pub mod physics;
pub mod player;

/// A random [`u64`] value used as the protocol ID version for the versions 0.1.x of the project.
pub const PROTOCOL_ID_V0_1: u64 = 1_542_994_232_742;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<OwnedByClient>()
            .replicate::<ReplicatedTransform>()
            .replicate::<PlayerAvatarColor>()
            .replicate::<PlayerDisplayName>()
            .add_client_event::<C2SHandshakeStart>(ChannelKind::Ordered)
            .add_server_event::<S2CHandshakeResult>(ChannelKind::Ordered)
            .add_server_event::<S2CSpawnPlayerCommand>(ChannelKind::Ordered)
            .add_client_event::<C2SInputEvent>(ChannelKind::Unreliable)
            .add_client_event::<C2SCommand>(ChannelKind::Ordered);
    }
}
