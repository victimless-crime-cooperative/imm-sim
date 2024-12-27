use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A small enum denoting how a digital input has been modulated since the last tick.
///
/// This allows the server to assume that, for example, a crouching player remains crouching should
/// a single message not be delivered.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum DigitalInput {
    #[default]
    NotPressed,
    StartPress,
    ContinuePress,
    ReleasePress,
}

/// Each tick, the client should dispatch one of these events such as to notify the server of all
/// inputs from the last tick.

#[derive(Clone, Copy, Debug, Deserialize, Event, Serialize)]
pub struct C2SInputEvent {
    /// Analog or digital input for movement along the "strafing" or "swaying" direction.
    ///
    /// This value may be any number from [-1.0, 1.0] such that:
    /// * A value of -1.0 will strafe the player to the left at maximum speed.
    /// * A value of 0.0 results in no movement left or right.
    /// * A value of 1.0 will strafe the player to the right at maximum speed.
    ///
    /// Pressing A or D (by default) will result in a value of -1.0 or 1.0 respectively. If neither
    /// button or both buttons are pressed, then a value of 0.0 will be sent.
    ///
    /// This is set up such as to ensure that analog movement with a joystick can be supported in
    /// the future.
    pub translation_strafe: f32,

    /// Analog or digital input for movement along the "walking" or "surging" direction.
    ///
    /// This value may be any number from [-1.0, 1.0] such that:
    /// * A value of -1.0 will move the player backwards at maximum speed.
    /// * A value of 0.0 results in no movement forwards or backwards.
    /// * A value of 1.0 will strafe the player forwards at maximum speed.
    ///
    /// Pressing S or W (by default) will result in a value of -1.0 or 1.0 respectively. If neither
    /// button or both buttons are pressed, then a value of 0.0 will be sent.
    ///
    /// This is set up such as to ensure that analog movement with a joystick can be supported in
    /// the future.
    pub translation_walk: f32,

    /// Analog input for a pitch rotation. This is an unbounded floating point value captured from
    /// mouse movement, and perhaps joysticks in the future.
    pub rotation_pitch: f32,

    /// Analog input for a yaw rotation. This is an unbounded floating point value captured from
    /// mouse movement, and perhaps joysticks in the futrue.
    pub rotation_yaw: f32,

    /// Digital input for a crouch button.
    pub crouch_button: DigitalInput,

    /// Digital input for a jump button.
    pub jump_button: DigitalInput,
}

#[derive(Clone, Copy, Debug, Deserialize, Event, Serialize)]
pub enum C2SCommand {
    ChangeAvatarColor { r: u8, g: u8, b: u8 },
}
