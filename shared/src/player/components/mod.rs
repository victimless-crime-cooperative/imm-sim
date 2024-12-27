use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Copy, Deserialize, PartialEq, Serialize)]
pub struct PlayerAvatarColor(pub Color);

#[derive(Clone, Component, Deserialize, PartialEq, Serialize)]
pub struct PlayerDisplayName(pub String);
