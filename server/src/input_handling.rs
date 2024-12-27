use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use imm_sim_shared::player::{
    components::PlayerAvatarColor,
    messages::client_input::{C2SCommand, C2SInputEvent},
};

use crate::connection_handling::ConnectionTracker;

// TODO: Make these constant values [`Component`]s
const MOVEMENT_ACCELERATION: f32 = 25.0;
const LATERAL_DAMPING: f32 = 0.8;
const JUMP_IMPULSE: f32 = 5.0;

pub fn handle_player_inputs(
    mut reader: EventReader<FromClient<C2SInputEvent>>,

    time: Res<Time>,
    conn_tracker: Res<ConnectionTracker>,

    mut player_query: Query<(&Transform, &mut LinearVelocity)>,
) {
    for FromClient { client_id, event } in reader.read() {
        let client_id = client_id.get();
        let Some(avatar) = conn_tracker.get_avatar(client_id) else {
            warn!("Received input from client {client_id} who is not in the connection tracker.");
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

        let (transform, mut lin_vel) = match player_query.get_mut(avatar) {
            Ok(query_res) => query_res,
            Err(e) => {
                error!("Player {client_id}'s avatar is missing a component: {e}");
                continue;
            }
        };

        // TODO: Telegraph player rotation somehow.
        // TODO: Handle crouch/jump

        let current_direction = transform.rotation;
        let movement_direction =
            current_direction * Vec3::new(*translation_strafe, 0.0, *translation_walk);

        lin_vel.0 += movement_direction * MOVEMENT_ACCELERATION * time.delta_secs();
    }
}

pub fn handle_player_commands(
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
