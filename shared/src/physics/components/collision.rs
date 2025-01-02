use avian3d::prelude::*;
use bevy::prelude::*;

/// Marker struct for the top of a players collider
#[derive(Component)]
pub struct PlayerTopCollider;
/// Marker struct for the bottom of a players collider
#[derive(Component)]
pub struct PlayerBottomCollider;

/// Collision layers
#[derive(Default, PhysicsLayer)]
pub enum CoLayer {
    #[default]
    Environment,
    Player,
    Pickup,
}
