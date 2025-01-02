use super::collision::PlayerTopCollider;
use avian3d::prelude::*;
use bevy::{ecs::component::StorageType, prelude::*};

pub struct Crouching;

impl Component for Crouching {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, entity, _| {
            let children = world.entity(entity).components::<&Children>();
            let mut head_entity: Option<Entity> = None;
            for child in children {
                let is_top_collider = world.entity(*child).contains::<PlayerTopCollider>();
                let has_sensor = world.entity(*child).contains::<Sensor>();

                if is_top_collider && has_sensor {
                    head_entity = Some(*child);
                }
            }
            if let Some(child_entity) = head_entity {
                world.commands().entity(child_entity).remove::<Sensor>();
            }
        });

        hooks.on_add(|mut world, entity, _| {
            let children = world.entity(entity).components::<&Children>();
            let mut head_entity: Option<Entity> = None;
            for child in children {
                let is_top_collider = world.entity(*child).contains::<PlayerTopCollider>();
                let has_sensor = world.entity(*child).contains::<Sensor>();

                if is_top_collider && !has_sensor {
                    head_entity = Some(*child);
                }
            }
            if let Some(child_entity) = head_entity {
                world.commands().entity(child_entity).insert(Sensor);
            }
        });
    }
}

pub struct Grounded;

impl Component for Grounded {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            if world.entity(entity).contains::<Crouching>() {
                world.commands().entity(entity).remove::<Crouching>();
            }
        });
    }
}
#[derive(Component)]
pub struct HeadBlocked;
#[derive(Component)]
pub struct MovementAcceleration(pub f32);
#[derive(Component)]
pub struct LateralDamping(pub f32);
#[derive(Component)]
pub struct JumpImpulse(pub f32);
#[derive(Component)]
pub struct SlopeData {
    pub ground_normal: Vec3,
}

impl SlopeData {
    pub fn get_slope_from_direction(&self, direction: Vec3) -> f32 {
        direction.dot(self.ground_normal)
    }
}

impl Default for SlopeData {
    fn default() -> Self {
        Self {
            ground_normal: Vec3::NEG_Y,
        }
    }
}

impl Default for MovementAcceleration {
    fn default() -> Self {
        Self(25.0)
    }
}

impl Default for LateralDamping {
    fn default() -> Self {
        Self(0.8)
    }
}

impl Default for JumpImpulse {
    fn default() -> Self {
        Self(5.0)
    }
}
