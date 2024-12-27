use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientTransport},
    renet::{ConnectionConfig, RenetClient},
};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::RenetChannelsExt;
use imm_sim_shared::{
    PROTOCOL_ID_V0_1,
    handshake::{C2SHandshakeStart, S2CHandshakeResult},
};

pub struct FormConnectionPlugin;

impl Plugin for FormConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ConnectionState>()
            .init_resource::<ConnectServerMenuInput>()
            .add_systems(
                Update,
                render_main_menu.run_if(in_state(ConnectionState::ConnectServerMenu)),
            )
            .add_systems(
                Update,
                render_connecting_screen.run_if(
                    in_state(ConnectionState::TryingConnection)
                        .or(in_state(ConnectionState::AwaitingHandshakeResponse)),
                ),
            )
            .add_systems(OnEnter(ConnectionState::TryingConnection), process_connect)
            .add_systems(
                Update,
                send_handshake
                    .run_if(in_state(ConnectionState::SendingHandshake).and(client_connected)),
            )
            .add_systems(
                Update,
                recv_handshake_result.run_if(in_state(ConnectionState::AwaitingHandshakeResponse)),
            );
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ConnectionState {
    #[default]
    ConnectServerMenu,
    TryingConnection,
    SendingHandshake,
    AwaitingHandshakeResponse,
    InGame,
}

#[derive(Resource)]
pub struct ClientId(pub u64);

#[derive(Default, Resource)]
struct ConnectServerMenuInput {
    pub error_message: Option<String>,

    pub server_address: String,
    pub server_password: String,
    pub display_name: String,
}

fn render_main_menu(
    mut contexts: EguiContexts,
    mut input: ResMut<ConnectServerMenuInput>,
    mut next: ResMut<NextState<ConnectionState>>,
) {
    egui::Window::new("Connect to Server").show(contexts.ctx_mut(), |ui| {
        if let Some(e) = input.error_message.as_ref() {
            ui.label(format!("Error connecting: {e}"));
        }

        ui.label("Server Address:");
        ui.text_edit_singleline(&mut input.server_address);

        ui.label("Server Password (Optional):");
        egui::TextEdit::singleline(&mut input.server_password)
            .password(true)
            .show(ui);

        ui.label("Display Name:");
        ui.text_edit_singleline(&mut input.display_name);

        if ui.button("Connect").clicked() {
            next.set(ConnectionState::TryingConnection);
        }
    });
}

fn render_connecting_screen(mut contexts: EguiContexts) {
    egui::Window::new("Connect to Server").show(contexts.ctx_mut(), |ui| {
        ui.label("Connection in progress...")
    });
}

fn process_connect(
    channels: Res<RepliconChannels>,
    mut input: ResMut<ConnectServerMenuInput>,
    mut next: ResMut<NextState<ConnectionState>>,
    mut commands: Commands,
) {
    let server_channels_config = channels.get_server_configs();
    let client_channels_config = channels.get_client_configs();

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let socket = match UdpSocket::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)) {
        Ok(socket) => socket,
        Err(e) => {
            input.error_message = Some(format!("Error binding to local socket: {e}"));
            next.set(ConnectionState::ConnectServerMenu);
            return;
        }
    };

    let server_addr = match SocketAddr::from_str(&input.server_address) {
        Ok(addr) => addr,
        Err(e) => {
            input.error_message = Some(format!("Error parsing the given server address: {e}"));
            next.set(ConnectionState::ConnectServerMenu);
            return;
        }
    };

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time is less than Unix epoch");
    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID_V0_1,
        client_id,
        server_addr,
        user_data: None,
    };

    let transport = match NetcodeClientTransport::new(current_time, authentication, socket) {
        Ok(transport) => transport,
        Err(e) => {
            input.error_message = Some(format!("Could not connect to server: {e}"));
            next.set(ConnectionState::ConnectServerMenu);
            return;
        }
    };

    commands.insert_resource(client);
    commands.insert_resource(transport);

    next.set(ConnectionState::SendingHandshake);
}

fn send_handshake(
    mut writer: EventWriter<C2SHandshakeStart>,
    input: Res<ConnectServerMenuInput>,
    mut next: ResMut<NextState<ConnectionState>>,
) {
    let event = C2SHandshakeStart {
        display_name: input.display_name.clone(),
        room_password: if input.server_password.is_empty() {
            None
        } else {
            Some(input.server_password.clone())
        },
    };

    writer.send(event);
    next.set(ConnectionState::AwaitingHandshakeResponse);
}

fn recv_handshake_result(
    mut reader: EventReader<S2CHandshakeResult>,
    mut input: ResMut<ConnectServerMenuInput>,
    mut next: ResMut<NextState<ConnectionState>>,
    mut commands: Commands,
) {
    for res in reader.read() {
        match res {
            S2CHandshakeResult::ConnectionAccepted { client_id } => {
                commands.insert_resource(ClientId(*client_id));
                next.set(ConnectionState::InGame)
            }
            S2CHandshakeResult::ConnectionRejected { reason } => {
                input.error_message = Some(format!(
                    "The server rejected your connection for reason: {reason}"
                ));
                next.set(ConnectionState::ConnectServerMenu);
            }
        }
    }
}
