use bevy::prelude::*;
use imm_sim_shared::physics::components::transform::ReplicatedTransform;

pub fn mirror_replicated_transform(mut query: Query<(&Transform, &mut ReplicatedTransform)>) {
    for (transform, mut replica) in query.iter_mut() {
        *replica = (*transform).into();
    }
}
