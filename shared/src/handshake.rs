use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Event, Serialize)]
pub struct C2SHandshakeStart {
    pub display_name: String,
    pub room_password: Option<String>,
}

#[derive(Debug, Deserialize, Event, Serialize)]
pub enum S2CHandshakeResult {
    ConnectionAccepted { client_id: u64 },
    ConnectionRejected { reason: String },
}
