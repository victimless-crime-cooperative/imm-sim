use bevy::prelude::*;

/// Event type that encompasses grounded, clientside movement actions
#[derive(Event)]
pub enum StandingAction {
    Run(Vec3),
    Jump,
    Crouch(Vec3),
    Uncrouch,
}

/// Event type that encompasses airborne, clientside movement actions
#[derive(Event)]
pub enum AirborneAction {
    Move(Vec3),
}
