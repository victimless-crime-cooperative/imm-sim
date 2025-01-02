use std::collections::{BTreeMap, HashMap};

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ConnectionTracker {
    conn_id_to_avatar: BTreeMap<u64, Entity>,

    conn_id_to_display_name: BTreeMap<u64, String>,
    display_name_to_conn_id: HashMap<String, u64>,
}

impl ConnectionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track_connection(&mut self, conn_id: u64, avatar: Entity, display_name: String) {
        self.conn_id_to_avatar.insert(conn_id, avatar);
        self.conn_id_to_display_name
            .insert(conn_id, display_name.clone());
        self.display_name_to_conn_id.insert(display_name, conn_id);
    }

    pub fn drop_connection(&mut self, conn_id: u64) -> Option<(Entity, String)> {
        let avatar = self.conn_id_to_avatar.remove(&conn_id)?;
        let display_name = self.conn_id_to_display_name.remove(&conn_id)?;

        let _ = self.display_name_to_conn_id.remove(&display_name);

        Some((avatar, display_name))
    }

    pub fn get_avatar(&self, conn_id: u64) -> Option<Entity> {
        self.conn_id_to_avatar.get(&conn_id).map(|entity| *entity)
    }

    pub fn get_display_name(&self, conn_id: u64) -> Option<&str> {
        self.conn_id_to_display_name
            .get(&conn_id)
            .map(String::as_str)
    }

    pub fn id_from_display_name(&self, name: &str) -> Option<u64> {
        self.display_name_to_conn_id.get(name).map(|id| *id)
    }
}
