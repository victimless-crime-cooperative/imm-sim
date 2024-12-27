use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A maker sutruct for "player" entities.
///
/// Every connection will have an associated avatar whose root [`Entity`] is marked with this
/// component.
#[derive(Clone, Component, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub struct Player;

/// A temporary component holding the [`Color`] that a given player avatar should be tinted.
#[derive(Clone, Component, Copy, Deserialize, PartialEq, Serialize)]
pub struct PlayerAvatarColor(pub Color);

/// A component denoting the display name of the connection associated with a given player entity.
#[derive(Clone, Component, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlayerDisplayName(pub String);
