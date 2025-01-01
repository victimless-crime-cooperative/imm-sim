use bevy::prelude::*;

/// Event type that encompasses all movement actions that can be done by a player on the ground
#[derive(Event)]
pub enum StandingAction {
    Run(Vec3),
    Jump,
    Crouch(Vec3),
    Uncrouch,
}
