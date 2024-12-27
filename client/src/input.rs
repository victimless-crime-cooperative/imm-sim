use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use bevy_replicon::prelude::*;
use imm_sim_shared::player::messages::client_input::{C2SCommand, C2SInputEvent, DigitalInput};

use crate::connect::ConnectionState;

// TODO: Make [`Component`]
pub const CAMERA_SENSITIVITY_X: f32 = 1.0;
pub const CAMERA_SENSITIVITY_Y: f32 = 1.0;

pub struct InputCollectionPlugin;

impl Plugin for InputCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (gather_input).run_if(in_state(ConnectionState::InGame)),
        );
    }
}

fn gather_input(
    mut writer: EventWriter<C2SInputEvent>,

    // mut last_inputs: ResMut<LastInputs>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_motion_input: Res<AccumulatedMouseMotion>,
) {
    let translation_strafe = {
        let mut strafe = 0.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            strafe -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            strafe += 1.0;
        }

        strafe
    };

    let translation_walk = {
        let mut walk = 0.0;

        if keyboard_input.pressed(KeyCode::KeyS) {
            walk -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            walk += 1.0;
        }

        walk
    };

    let Vec2 { x, y } = mouse_motion_input.delta;

    let rotation_pitch = time.delta_secs() * CAMERA_SENSITIVITY_Y * 45.0 * -y;
    let rotation_yaw = time.delta_secs() * CAMERA_SENSITIVITY_X * 45.0 * -x;

    let crouch_button = to_digital_input(&keyboard_input, KeyCode::ControlLeft);
    let jump_button = to_digital_input(&keyboard_input, KeyCode::Space);

    let input = C2SInputEvent {
        translation_strafe,
        translation_walk,
        rotation_pitch,
        rotation_yaw,
        crouch_button,
        jump_button,
    };

    writer.send(input);
}

fn to_digital_input(inputs: &ButtonInput<KeyCode>, code: KeyCode) -> DigitalInput {
    if inputs.just_pressed(code) {
        DigitalInput::StartPress
    } else if inputs.pressed(code) {
        DigitalInput::ContinuePress
    } else if inputs.just_released(code) {
        DigitalInput::ReleasePress
    } else {
        DigitalInput::NotPressed
    }
}
