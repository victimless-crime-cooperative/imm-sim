use std::collections::BTreeSet;

use bevy::prelude::*;
use bevy_replicon::prelude::*;
use imm_sim_shared::{
    handshake::{C2SHandshakeStart, S2CHandshakeResult},
    player::SpawnPlayerCommandsExt,
};
use rand::{Rng, thread_rng};

use super::tracking::ConnectionTracker;
use crate::RoomAuthentication;

#[derive(Default, Resource)]
pub struct AwaitingHandshakes {
    set: BTreeSet<u64>,
}

pub fn handle_connection_events(
    mut reader: EventReader<ServerEvent>,

    mut awaiting_handshakes: ResMut<AwaitingHandshakes>,
    mut conn_tracker: ResMut<ConnectionTracker>,

    mut commands: Commands,
) {
    for event in reader.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let client_id = client_id.get();
                info!("Client {client_id} has successfully connected to the server.");
                awaiting_handshakes.set.insert(client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                let client_id = client_id.get();
                match conn_tracker.drop_connection(client_id) {
                    Some((avatar, display_name)) => {
                        info!("{display_name} disconnected from the server for {reason}.");
                        commands.entity(avatar).despawn_recursive();
                    }

                    None => {
                        info!("Untracked connection {client_id} disconnected for {reason}.");
                    }
                }
            }
        }
    }
}

pub fn handle_handshake_events(
    mut reader: EventReader<FromClient<C2SHandshakeStart>>,
    mut writer: EventWriter<ToClients<S2CHandshakeResult>>,

    authentication: Res<RoomAuthentication>,
    mut awaiting_handshakes: ResMut<AwaitingHandshakes>,
    mut conn_tracker: ResMut<ConnectionTracker>,

    mut commands: Commands,
) {
    for FromClient {
        client_id,
        event: C2SHandshakeStart {
            display_name,
            room_password,
        },
    } in reader.read()
    {
        // If there is a password, reject the client if no password was given with the handshake, or
        // if the password given is incorrect.
        if let RoomAuthentication::WithPassword(password) = authentication.as_ref() {
            if let Some(password_attempt) = room_password {
                if password_attempt != password {
                    let event = S2CHandshakeResult::ConnectionRejected {
                        reason: "The password you gave is not correct.".to_owned(),
                    };
                    let event = ToClients {
                        mode: SendMode::Direct(*client_id),
                        event,
                    };

                    writer.send(event);
                    continue;
                }
            } else {
                let event = S2CHandshakeResult::ConnectionRejected {
                    reason: "This server requires a password to join.".to_owned(),
                };
                let event = ToClients {
                    mode: SendMode::Direct(*client_id),
                    event,
                };

                writer.send(event);
                continue;
            }
        }

        // Should the password not be required, or be correct, then ensure that the display name
        // given is not already in use.
        if let Some(_id) = conn_tracker.id_from_display_name(display_name.as_str()) {
            let event = S2CHandshakeResult::ConnectionRejected {
                reason: format!(
                    "The requested display name `{display_name}`` is already in use on this server."
                ),
            };
            let event = ToClients {
                mode: SendMode::Direct(*client_id),
                event,
            };

            writer.send(event);
            continue;
        }

        // If all checks pass, send the `ConnectionAccepted` response to the client, spawn a player
        // for this new connection, begin tracking and replicating all relevant information.
        let event = S2CHandshakeResult::ConnectionAccepted {
            client_id: client_id.get(),
        };
        let event = ToClients {
            mode: SendMode::Direct(*client_id),
            event,
        };

        writer.send(event);

        awaiting_handshakes.set.remove(&client_id.get());

        let translation = {
            let mut rng = thread_rng();
            let x: f32 = rng.gen_range(-20.0..20.0);
            let y: f32 = 30.0;
            let z: f32 = rng.gen_range(-20.0..20.0);

            Vec3::new(x, y, z)
        };

        let color = {
            let mut rng = thread_rng();
            let r: u8 = rng.r#gen();
            let g: u8 = rng.r#gen();
            let b: u8 = rng.r#gen();

            Color::srgb_u8(r, g, b)
        };

        let entity_id = commands
            .spawn_player(
                client_id.get(),
                display_name.clone(),
                translation,
                Quat::IDENTITY,
                color,
            )
            .id();

        conn_tracker.track_connection(client_id.get(), entity_id, display_name.clone());
    }
}
