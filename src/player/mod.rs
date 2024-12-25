use crate::camera::CameraConfig;
use crate::physics::{JumpImpulse, LateralDamping, MovementAcceleration, StandingAction};
use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (read_movement_inputs, handle_grounded));
    }
}

#[derive(Component)]
pub struct PlayerTopCollider;
#[derive(Component)]
pub struct PlayerBottomCollider;
#[derive(Component, Default)]
pub struct Player {
    pub height: f32,
}

#[derive(Component, Default)]
pub enum PlayerState {
    Standing,
    Crouching,
    #[default]
    Airborne,
}

pub struct SpawnPlayer {
    translation: Vec3,
    height: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world
            .spawn((
                Player {
                    height: self.height,
                },
                PlayerState::default(),
                RigidBody::Dynamic,
                Transform::from_translation(self.translation),
                LockedAxes::ROTATION_LOCKED,
                JumpImpulse::default(),
                MovementAcceleration::default(),
                LateralDamping::default(),
                ShapeCaster::new(
                    Collider::capsule(0.2, 1.0),
                    Vec3::ZERO,
                    Quat::IDENTITY,
                    Dir3::NEG_Y,
                )
                .with_ignore_self(true)
                .with_max_distance(1.0),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Transform::from_translation(Vec3::Y * self.height * 0.75),
                    PlayerTopCollider,
                    Collider::sphere(0.25),
                ));
                parent.spawn((
                    Transform::from_translation(Vec3::Y * self.height * 0.25),
                    PlayerBottomCollider,
                    Collider::sphere(0.25),
                ));
            });
    }
}

fn setup(mut commands: Commands) {
    commands.queue(SpawnPlayer {
        height: 2.0,
        translation: Vec3::NEG_Z + Vec3::Y * 30.0,
    });
}

fn read_movement_inputs(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<Entity, With<Player>>,
) {
    let player_entity = player.into_inner();
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;

    let direction = camera_config.interpolate(Vec2::new(horizontal as f32, vertical as f32));

    if direction != Vec3::ZERO {
        commands.trigger_targets(StandingAction::Run(direction), player_entity);
    }
}

fn handle_grounded(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&ShapeHits, &mut PlayerState)>,
) {
    if let Ok((hits, mut player_state)) = player.get_single_mut() {
        if !hits.is_empty() {
            if input.pressed(KeyCode::KeyQ) {
                *player_state = PlayerState::Crouching;
            } else {
                *player_state = PlayerState::Standing;
            }
        } else {
            *player_state = PlayerState::Airborne;
        }
    }
}
