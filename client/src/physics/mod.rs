use bevy::prelude::*;
use bevy_replicon::client::ClientSet;
use imm_sim_shared::physics::components::transform::ReplicatedTransform;

use crate::connect::ConnectionState;

pub struct ClientPhysicsPlugin;

impl Plugin for ClientPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            mirror_transforms
                .run_if(in_state(ConnectionState::InGame))
                .after(ClientSet::Receive),
        );
    }
}

fn mirror_transforms(mut query: Query<(&ReplicatedTransform, &mut Transform)>) {
    for (replica, mut transform) in query.iter_mut() {
        *transform = (*replica).into();
    }
}
