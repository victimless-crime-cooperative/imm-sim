use std::net::SocketAddr;

use bevy::prelude::*;
use clap::Parser;
use imm_sim_server::{ImmSimServerPlugin, ServerLifecycleCmd};

#[derive(Parser)]
pub struct Args {
    bind_addr: SocketAddr,
    room_password: Option<String>,
}

fn main() {
    App::new()
        .add_plugins(ImmSimServerPlugin::standalone())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut writer: EventWriter<ServerLifecycleCmd>) {
    let Args {
        bind_addr,
        room_password,
    } = Args::parse();

    writer.send(ServerLifecycleCmd::StartServer {
        bind_addr,
        room_password,
    });
}
