use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    utils::{HashMap, hashbrown::hash_map::Entry},
};
use imm_sim_shared::player::messages::client_input::{C2SInputEvent, DigitalInput};

use crate::connect::ConnectionState;

// TODO: Make [`Resource`]
pub const CAMERA_SENSITIVITY_X: f32 = 1.0;
pub const CAMERA_SENSITIVITY_Y: f32 = 1.0;

pub struct InputCollectionPlugin;

impl Plugin for InputCollectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseInputAcc>()
            .init_resource::<KeyboardInputAcc>()
            .add_systems(
                Update,
                gather_input.run_if(in_state(ConnectionState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                send_input.run_if(in_state(ConnectionState::InGame)),
            );
    }
}

#[derive(Default, Resource)]
pub struct MouseInputAcc {
    rotation_pitch: f32,
    rotation_yaw: f32,
}

impl MouseInputAcc {
    fn clear(&mut self) {
        self.rotation_pitch = 0.0;
        self.rotation_yaw = 0.0;
    }
}

#[derive(Default, Resource)]
pub struct KeyboardInputAcc {
    map: HashMap<KeyCode, DigitalInput>,
}

impl KeyboardInputAcc {
    fn record(&mut self, keycode: KeyCode, input: DigitalInput) {
        match self.map.entry(keycode) {
            Entry::Vacant(vacancy) => {
                vacancy.insert(input);
            }

            // If an input has already been recorded, prioritize the "more significant" variant.
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if *entry < input {
                    *entry = input;
                }
            }
        }
    }

    fn get(&self, keycode: &KeyCode) -> DigitalInput {
        self.map.get(keycode).map(|i| *i).unwrap_or_default()
    }

    fn clear(&mut self) {
        self.map.clear();
    }
}

fn gather_input(
    mut acc_mouse: ResMut<MouseInputAcc>,
    mut acc_keyboard: ResMut<KeyboardInputAcc>,

    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_motion_input: Res<AccumulatedMouseMotion>,
) {
    let Vec2 { x, y } = mouse_motion_input.delta;

    let rotation_pitch = time.delta_secs() * CAMERA_SENSITIVITY_Y * 45.0 * -y;
    let rotation_yaw = time.delta_secs() * CAMERA_SENSITIVITY_X * 45.0 * -x;

    acc_mouse.rotation_pitch += rotation_pitch;
    acc_mouse.rotation_yaw += rotation_yaw;

    for key in keyboard_input.get_just_pressed() {
        acc_keyboard.record(*key, DigitalInput::StartPress);
    }

    for key in keyboard_input.get_just_released() {
        acc_keyboard.record(*key, DigitalInput::ReleasePress);
    }

    for key in keyboard_input.get_pressed() {
        acc_keyboard.record(*key, DigitalInput::ContinuePress);
    }
}

fn send_input(
    mut writer: EventWriter<C2SInputEvent>,

    mut acc_mouse: ResMut<MouseInputAcc>,
    mut acc_keyboard: ResMut<KeyboardInputAcc>,
) {
    let translation_strafe = {
        let mut strafe = 0.0;

        if acc_keyboard.get(&KeyCode::KeyA).is_pressed() {
            strafe -= 1.0;
        }

        if acc_keyboard.get(&KeyCode::KeyD).is_pressed() {
            strafe += 1.0;
        }

        strafe
    };

    let translation_walk = {
        let mut walk = 0.0;

        if acc_keyboard.get(&KeyCode::KeyS).is_pressed() {
            walk -= 1.0;
        }

        if acc_keyboard.get(&KeyCode::KeyW).is_pressed() {
            walk += 1.0;
        }

        walk
    };

    let rotation_pitch = acc_mouse.rotation_pitch;
    let rotation_yaw = acc_mouse.rotation_yaw;

    let crouch_button = acc_keyboard.get(&KeyCode::ControlLeft);
    let jump_button = acc_keyboard.get(&KeyCode::Space);

    let input = C2SInputEvent {
        translation_strafe,
        translation_walk,
        rotation_pitch,
        rotation_yaw,
        crouch_button,
        jump_button,
    };

    writer.send(input);

    acc_keyboard.clear();
    acc_mouse.clear();
}
