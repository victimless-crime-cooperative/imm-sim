use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct MovementAcceleration(pub f32);
#[derive(Component)]
pub struct LateralDamping(pub f32);
#[derive(Component)]
pub struct JumpImpulse(pub f32);
