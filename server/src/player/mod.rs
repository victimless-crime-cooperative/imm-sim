use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use imm_sim_shared::player::{
    components::PlayerAvatarColor,
    messages::client_input::{C2SCommand, C2SInputEvent},
};

use crate::{ServerState, connection::tracking::ConnectionTracker};

pub struct ServerPlayerPlugin;

const MOVEMENT_ACCELERATION: f32 = 25.0;

impl Plugin for ServerPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            handle_player_inputs.run_if(in_state(ServerState::Running)),
        );

        app.add_systems(
            FixedUpdate,
            handle_player_commands.run_if(in_state(ServerState::Running)),
        );
    }
}

fn handle_player_inputs(
    mut reader: EventReader<FromClient<C2SInputEvent>>,

    time: Res<Time>,
    conn_tracker: Res<ConnectionTracker>,

    mut query: Query<(&Transform, &mut LinearVelocity)>,
) {
    for FromClient { client_id, event } in reader.read() {
        let client_id = client_id.get();
        let Some(avatar) = conn_tracker.get_avatar(client_id) else {
            debug!("Unexepected input from client {client_id}. This client is not tracked.");
            continue;
        };

        let C2SInputEvent {
            translation_strafe,
            translation_walk,
            // rotation_pitch,
            // rotation_yaw,
            // crouch_button,
            // jump_button,
            ..
        } = event;

        let (transform, mut lin_vel) = match query.get_mut(avatar) {
            Ok(out) => out,
            Err(e) => {
                error!("Player {client_id}'s avatar is missing a component: {e}");
                continue;
            }
        };

        let current_direction = transform.rotation;
        let movement_direction =
            current_direction * Vec3::new(*translation_strafe, 0.0, *translation_walk);

        lin_vel.0 += movement_direction * MOVEMENT_ACCELERATION * time.delta_secs();
    }
}

fn handle_player_commands(
    mut reader: EventReader<FromClient<C2SCommand>>,
    conn_tracker: Res<ConnectionTracker>,
    mut color_query: Query<&mut PlayerAvatarColor>,
) {
    for FromClient { client_id, event } in reader.read() {
        let client_id = client_id.get();
        let Some(avatar) = conn_tracker.get_avatar(client_id) else {
            warn!("Received command from client {client_id} who is not in the connection tracker.");
            continue;
        };

        match event {
            C2SCommand::ChangeAvatarColor { r, g, b } => {
                let color = Color::srgb_u8(*r, *g, *b);
                let Ok(mut color_component) = color_query.get_mut(avatar) else {
                    error!(
                        "Player {client_id}'s avatar is missing a `PlayerAvatarColor` component."
                    );
                    continue;
                };

                color_component.0 = color;
            }
        }
    }
}
