use avian3d::prelude::*;
use bevy::prelude::*;
use imm_sim_shared::physics::components::collision::{
    CoLayer, PlayerBottomCollider, PlayerTopCollider,
};
pub fn generate_collision_components(height: f32) -> (ShapeCaster, impl Bundle, impl Bundle) {
    let collision_sphere = Collider::sphere(height * 0.25);
    let shape_caster = ShapeCaster::new(
        Collider::sphere(height * 0.15),
        Vec3::NEG_Y * (height * 0.4),
        Quat::IDENTITY,
        Dir3::NEG_Y,
    )
    .with_ignore_self(true)
    .with_max_distance(height * 0.6)
    .with_query_filter(SpatialQueryFilter::default().with_mask(CoLayer::Environment));

    let top_collider = (
        Transform::from_translation(Vec3::Y * (height * 0.25)),
        collision_sphere.clone(),
        PlayerTopCollider,
        generate_collision_layers(),
    );

    let bottom_collider = (
        Transform::from_translation(Vec3::NEG_Y * (height * 0.25)),
        PlayerBottomCollider,
        collision_sphere,
        generate_collision_layers(),
    );

    (shape_caster, top_collider, bottom_collider)
}

pub fn generate_collision_layers() -> CollisionLayers {
    CollisionLayers::new(
        CoLayer::Player,
        [CoLayer::Player, CoLayer::Environment, CoLayer::Pickup],
    )
}
