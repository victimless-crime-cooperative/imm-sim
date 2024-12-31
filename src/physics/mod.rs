use avian3d::prelude::*;
use bevy::prelude::*;

use crate::player::PlayerTopCollider;

pub struct CharacterPhysicsPlugin;

impl Plugin for CharacterPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_movement_damping, track_head_blockage))
            .add_observer(execute_standing_actions);
    }
}

#[derive(Default, PhysicsLayer)]
pub enum CoLayer {
    #[default]
    Environment,
    Player,
    Pickup,
}

#[derive(Component)]
pub struct HeadBlocked;
#[derive(Component)]
pub struct MovementAcceleration(pub f32);
#[derive(Component)]
pub struct LateralDamping(pub f32);
#[derive(Component)]
pub struct JumpImpulse(pub f32);

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

#[derive(Event)]
pub enum StandingAction {
    Run(Vec3),
    Jump,
}

fn execute_standing_actions(
    trigger: Trigger<StandingAction>,
    time: Res<Time>,
    mut query: Query<(&MovementAcceleration, &JumpImpulse, &mut LinearVelocity)>,
) {
    if let Ok((movement_acceleration, jump_impulse, mut linear_velocity)) =
        query.get_mut(trigger.entity())
    {
        match trigger.event() {
            StandingAction::Run(direction) => {
                linear_velocity.0 += direction * movement_acceleration.0 * time.delta_secs();
            }
            StandingAction::Jump => linear_velocity.y = jump_impulse.0,
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

fn apply_movement_damping(mut query: Query<(&LateralDamping, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
