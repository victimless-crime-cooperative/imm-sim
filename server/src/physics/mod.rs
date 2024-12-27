use bevy::prelude::*;
use bevy_replicon::server::ServerSet;
use imm_sim_shared::physics::components::transform::ReplicatedTransform;

use crate::ServerState;

pub struct ServerPhysicsPlugin;

impl Plugin for ServerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            mirror_transforms
                .run_if(in_state(ServerState::Running))
                .before(ServerSet::Send),
        );
    }
}

fn mirror_transforms(mut query: Query<(&Transform, &mut ReplicatedTransform)>) {
    for (transform, mut replica) in query.iter_mut() {
        *replica = (*transform).into();
    }
}
