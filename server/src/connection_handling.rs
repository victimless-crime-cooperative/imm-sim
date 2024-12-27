use std::collections::{BTreeMap, BTreeSet, HashMap};

use bevy::prelude::*;
use bevy_renet::renet::ServerEvent;
use bevy_replicon::prelude::*;
use imm_sim_shared::{
    self,
    handshake::{C2SHandshakeStart, S2CHandshakeResult},
    player::{SpawnPlayerCommandsExt, messages::server_commands::S2CSpawnPlayerCommand},
};
use rand::{Rng, thread_rng};

use crate::RoomAuthentication;

#[derive(Default, Resource)]
pub struct ConnectionTracker {
    conn_id_to_avatar: BTreeMap<u64, Entity>,

    conn_id_to_display_name: BTreeMap<u64, String>,
    display_name_to_conn_id: HashMap<String, u64>,
}

impl ConnectionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track_connection(&mut self, conn_id: u64, avatar: Entity, display_name: String) {
        self.conn_id_to_avatar.insert(conn_id, avatar);
        self.conn_id_to_display_name
            .insert(conn_id, display_name.clone());
        self.display_name_to_conn_id.insert(display_name, conn_id);
    }

    pub fn drop_connection(&mut self, conn_id: u64) -> Option<(Entity, String)> {
        let avatar = self.conn_id_to_avatar.remove(&conn_id)?;
        let display_name = self.conn_id_to_display_name.remove(&conn_id)?;

        let _ = self.display_name_to_conn_id.remove(&display_name);

        Some((avatar, display_name))
    }

    pub fn get_avatar(&self, conn_id: u64) -> Option<Entity> {
        self.conn_id_to_avatar.get(&conn_id).map(|entity| *entity)
    }

    pub fn get_display_name(&self, conn_id: u64) -> Option<&str> {
        self.conn_id_to_display_name
            .get(&conn_id)
            .map(String::as_str)
    }

    pub fn id_from_display_name(&self, name: &str) -> Option<u64> {
        self.display_name_to_conn_id.get(name).map(|id| *id)
    }
}

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
                info!("Client {client_id} has successfully connected to the server.");
                awaiting_handshakes.set.insert(*client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                match conn_tracker.drop_connection(*client_id) {
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
    mut handshake_writer: EventWriter<ToClients<S2CHandshakeResult>>,
    mut client_spawn_writer: EventWriter<ToClients<S2CSpawnPlayerCommand>>,

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

                    handshake_writer.send(event);
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

                handshake_writer.send(event);
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

            handshake_writer.send(event);
            continue;
        }

        // If all checks pass, send the `ConnectionAccepted` response to the client, spawn a player
        // for this new connection, begin tracking and replicating all relevant information, and
        // send the command to spawn the player on all clients.
        let event = S2CHandshakeResult::ConnectionAccepted {
            client_id: client_id.get(),
        };
        let event = ToClients {
            mode: SendMode::Direct(*client_id),
            event,
        };

        handshake_writer.send(event);

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

        let event = S2CSpawnPlayerCommand {
            for_client_id: client_id.get(),
            display_name: display_name.clone(),
            initial_translation: translation,
            initial_rotation: Quat::IDENTITY,
            initial_color: color,
        };

        let entity_id = commands.spawn_player_server(&event).id();
        conn_tracker.track_connection(client_id.get(), entity_id, display_name.clone());

        let event = ToClients {
            mode: SendMode::Broadcast,
            event,
        };

        client_spawn_writer.send(event);
    }
}
