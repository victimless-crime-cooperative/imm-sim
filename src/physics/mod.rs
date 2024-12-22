use avian3d::prelude::*;
use bevy::prelude::*;

pub struct CharacterPhysicsPlugin;

impl Plugin for CharacterPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement_damping)
            .add_observer(execute_standing_actions);
    }
}

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

fn apply_movement_damping(mut query: Query<(&LateralDamping, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}
