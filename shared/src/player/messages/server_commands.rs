use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Event, Serialize)]
pub struct S2CSpawnPlayerCommand {
    pub for_client_id: u64,
    pub display_name: String,

    pub initial_translation: Vec3,
    pub initial_rotation: Quat,

    pub initial_color: Color,
}
