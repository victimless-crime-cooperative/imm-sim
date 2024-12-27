use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use avian3d::PhysicsPlugins;
use bevy::{
    app::{PanicHandlerPlugin, TerminalCtrlCHandlerPlugin},
    diagnostic::DiagnosticsPlugin,
    log::LogPlugin,
    prelude::*,
    scene::ScenePlugin,
    state::app::StatesPlugin,
};
use bevy_renet::{
    netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, RenetServer},
};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::{RenetChannelsExt, RepliconRenetPlugins};
use imm_sim_shared::{PROTOCOL_ID_V0_1, ProtocolPlugin};

use self::{
    connection::{
        ServerConnectionsPlugin, handle_incoming::AwaitingHandshakes, tracking::ConnectionTracker,
    },
    physics::ServerPhysicsPlugin,
    player::ServerPlayerPlugin,
};

mod connection;
mod physics;
mod player;

/// Whether the server is running as a standalone process, or within a client binary.
enum ServerRunMode {
    Standalone,
    WithClient,
}

/// This [`Plugin`] will allow for the creation and launch a server.
///
/// Some logic will be omitted when the given `run_mode` is [`ServerRunMode::WithClient`] to prevent
/// the duplication of certain plugins.
///
/// NOTE: The server will not start until a [`ServerLifecycleCmd::StartServer`] event has been
/// dispatched. Most interaction with the server systems will be through events.
pub struct ImmSimServerPlugin {
    run_mode: ServerRunMode,
}

impl ImmSimServerPlugin {
    pub fn standalone() -> Self {
        Self {
            run_mode: ServerRunMode::Standalone,
        }
    }

    pub fn alongside_client() -> Self {
        Self {
            run_mode: ServerRunMode::WithClient,
        }
    }
}

impl Plugin for ImmSimServerPlugin {
    fn build(&self, app: &mut App) {
        if matches!(self.run_mode, ServerRunMode::Standalone) {
            // [`MinimalPlugins`] + all the [`DefaultPlugins`] memembers that are relevant.
            app.add_plugins((
                MinimalPlugins,
                PanicHandlerPlugin,
                LogPlugin::default(),
                TransformPlugin,
                HierarchyPlugin,
                DiagnosticsPlugin,
                TerminalCtrlCHandlerPlugin,
                AssetPlugin::default(),
                ScenePlugin,
                StatesPlugin,
            ))
            // Networking plugins
            .add_plugins((RepliconPlugins, RepliconRenetPlugins))
            // Custom protocol plugin
            .add_plugins(ProtocolPlugin)
            // Physics plugin
            .add_plugins(PhysicsPlugins::default())
            // Netcode and physics should be on a fixed time-step
            .insert_resource(Time::<Fixed>::from_hz(30.0));
        }

        // Server lifecycle functionality.
        app.add_event::<ServerLifecycleCmd>()
            .init_state::<ServerState>()
            .add_systems(Update, listen_lifecycle_cmd)
            .add_systems(OnEnter(ServerState::Running), start_server)
            .add_systems(OnEnter(ServerState::Stopped), stop_server);

        // Handle connections and handshakes
        app.add_plugins(ServerConnectionsPlugin);

        // Handle player inputs and commands
        app.add_plugins(ServerPlayerPlugin);

        // State sync
        app.add_plugins(ServerPhysicsPlugin);
    }
}

#[derive(Event)]
pub enum ServerLifecycleCmd {
    StartServer {
        bind_addr: SocketAddr,
        room_password: Option<String>,
    },
    StopServer,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum ServerState {
    #[default]
    NotRunning,
    Running,
    Stopped,
    Errored,
}

#[derive(Resource)]
struct BindAddr(pub SocketAddr);

#[derive(Resource)]
enum RoomAuthentication {
    WithPassword(String),
    WithoutPassword,
}

fn listen_lifecycle_cmd(
    mut reader: EventReader<ServerLifecycleCmd>,
    mut next: ResMut<NextState<ServerState>>,

    mut commands: Commands,
) {
    for event in reader.read() {
        match event {
            ServerLifecycleCmd::StartServer {
                bind_addr,
                room_password,
            } => {
                commands.insert_resource(BindAddr(*bind_addr));

                let auth = if let Some(pass) = room_password {
                    RoomAuthentication::WithPassword(pass.clone())
                } else {
                    RoomAuthentication::WithoutPassword
                };
                commands.insert_resource(auth);

                next.set(ServerState::Running);
            }

            ServerLifecycleCmd::StopServer => {
                next.set(ServerState::Stopped);
            }
        }
    }
}

fn start_server(
    bind_addr: Res<BindAddr>,
    channels: Res<RepliconChannels>,
    mut next: ResMut<NextState<ServerState>>,
    mut commands: Commands,
) {
    let mut err_closure = |e| {
        error!("Error starting server: {e}");
        next.set(ServerState::Errored);
    };

    commands.init_resource::<AwaitingHandshakes>();
    commands.init_resource::<ConnectionTracker>();

    let server_channels_config = channels.get_server_configs();
    let client_channels_config = channels.get_client_configs();

    let server = RenetServer::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let socket = match UdpSocket::bind(bind_addr.0) {
        Ok(socket) => socket,
        Err(e) => {
            err_closure(e);
            return;
        }
    };

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time is less than Unix epoch");

    let server_config = ServerConfig {
        current_time,
        max_clients: 32,
        protocol_id: PROTOCOL_ID_V0_1,
        public_addresses: vec![bind_addr.0],
        // FIXME: Make secure in the final version.
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = match NetcodeServerTransport::new(server_config, socket) {
        Ok(transport) => transport,
        Err(e) => {
            err_closure(e);
            return;
        }
    };

    commands.insert_resource(server);
    commands.insert_resource(transport);

    next.set(ServerState::Running);
}

fn stop_server(mut server: ResMut<RenetServer>) {
    server.disconnect_all();
}
