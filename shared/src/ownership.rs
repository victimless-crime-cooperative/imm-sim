use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marks that a given entity is "owned by" the client with the given ID.
///
/// For now, it is just the player's avatar which is given this component.
#[derive(Clone, Component, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OwnedByClient {
    pub client_id: u64,
}
