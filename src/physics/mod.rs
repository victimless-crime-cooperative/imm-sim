use avian3d::prelude::*;
use bevy::ecs::component::StorageType;
use bevy::prelude::*;

use crate::actions::{AirborneAction, StandingAction};
use crate::player::PlayerTopCollider;

pub struct CharacterPhysicsPlugin;

impl Plugin for CharacterPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_movement_damping, track_head_blockage, handle_grounded),
        )
        .add_observer(execute_grounded_movement_actions)
        .add_observer(execute_airborne_movement_actions);
    }
}

#[derive(Default, PhysicsLayer)]
pub enum CoLayer {
    #[default]
    Environment,
    Player,
    Pickup,
}

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

fn execute_grounded_movement_actions(
    trigger: Trigger<StandingAction>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &MovementAcceleration,
            &JumpImpulse,
            &SlopeData,
            &mut LinearVelocity,
            Has<Crouching>,
            Has<HeadBlocked>,
        ),
        With<Grounded>,
    >,
) {
    if let Ok((
        entity,
        movement_acceleration,
        jump_impulse,
        slope,
        mut linear_velocity,
        has_crouching,
        has_headblocked,
    )) = query.get_mut(trigger.entity())
    {
        match trigger.event() {
            StandingAction::Run(direction) => {
                linear_velocity.0 += direction * movement_acceleration.0 * time.delta_secs();
            }
            StandingAction::Jump => linear_velocity.y = jump_impulse.0,
            StandingAction::Crouch(direction) => {
                if *direction == Vec3::ZERO {
                    if !has_crouching {
                        commands.entity(entity).insert(Crouching);
                    }
                } else {
                    let effective_slope = slope.get_slope_from_direction(*direction);
                    if effective_slope >= 0.0 && !has_crouching {
                        commands.entity(entity).insert(Crouching);
                    } else {
                    }
                }
            }
            StandingAction::Uncrouch => {
                if has_crouching && !has_headblocked {
                    commands.entity(entity).remove::<Crouching>();
                }
            }
        }
    }
}
fn execute_airborne_movement_actions(
    trigger: Trigger<AirborneAction>,
    time: Res<Time>,
    mut query: Query<(&MovementAcceleration, &mut LinearVelocity), Without<Grounded>>,
) {
    if let Ok((movement_acceleration, mut linear_velocity)) = query.get_mut(trigger.entity()) {
        match trigger.event() {
            AirborneAction::Move(direction) => {
                linear_velocity.0 += direction * movement_acceleration.0 * time.delta_secs();
            }
        }
    }
}

fn track_head_blockage(
    mut commands: Commands,
    collisions: Res<Collisions>,
    parent_query: Query<Has<HeadBlocked>, Without<PlayerTopCollider>>,
    player_head_query: Query<(Entity, &Parent, Has<Sensor>), With<PlayerTopCollider>>,
) {
    for (head_entity, parent, has_sensor) in &player_head_query {
        let has_headblocked = parent_query.get(parent.get()).unwrap();
        if collisions
            .collisions_with_entity(head_entity)
            .next()
            .is_some()
        {
            if !has_headblocked && has_sensor {
                commands.entity(parent.get()).insert(HeadBlocked);
            }
        } else {
            if has_headblocked {
                commands.entity(parent.get()).remove::<HeadBlocked>();
            }
        }
    }
}

fn handle_grounded(
    mut commands: Commands,
    mut player: Query<(Entity, &ShapeHits, &mut SlopeData, Has<Grounded>)>,
) {
    if let Ok((entity, hits, mut slope_data, has_grounded)) = player.get_single_mut() {
        let is_grounded = hits.iter().any(|hit| {
            slope_data.ground_normal = hit.normal2;
            true
        });

        if is_grounded && !has_grounded {
            commands.entity(entity).insert(Grounded);
        } else if !is_grounded && has_grounded {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn apply_movement_damping(mut query: Query<(&LateralDamping, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
