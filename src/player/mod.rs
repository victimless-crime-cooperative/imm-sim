use crate::camera::CameraConfig;
use crate::physics::{
    HeadBlocked, JumpImpulse, LateralDamping, MovementAcceleration, StandingAction,
};
use avian3d::prelude::*;
use bevy::prelude::*;

mod collision;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (read_movement_inputs, handle_grounded, handle_head_collider),
            )
            .register_type::<PlayerState>();
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

#[derive(Component, Default, Reflect, Eq, PartialEq)]
#[reflect(Component)]
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
        let (shape_caster, player_top, player_bottom) =
            collision::generate_collision_components(self.height);
        let collision_layers = collision::generate_collision_layers();
        world
            .spawn((
                Name::from("Player"),
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
                shape_caster,
                collision_layers,
            ))
            .with_children(|parent| {
                parent.spawn(player_top);
                parent.spawn(player_bottom);
            });
    }
}

fn setup(mut commands: Commands) {
    commands.queue(SpawnPlayer {
        height: 1.0,
        translation: Vec3::NEG_Z + Vec3::Y * 30.0,
    });
}

fn read_movement_inputs(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(Entity, &PlayerState), With<Player>>,
) {
    let (player_entity, state) = player.into_inner();
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let space = keyboard_input.pressed(KeyCode::Space);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;

    let direction = camera_config.interpolate(Vec2::new(horizontal as f32, vertical as f32));

    if direction != Vec3::ZERO {
        commands.trigger_targets(StandingAction::Run(direction), player_entity);
    }

    if space && *state != PlayerState::Airborne {
        commands.trigger_targets(StandingAction::Jump, player_entity);
    }
}

fn handle_head_collider(
    mut commands: Commands,
    player_query: Query<&PlayerState>,
    head_query: Query<(Entity, Has<Sensor>), With<PlayerTopCollider>>,
) {
    for player in &player_query {
        for (head, has_sensor) in &head_query {
            match *player {
                PlayerState::Crouching => {
                    if !has_sensor {
                        commands.entity(head).insert(Sensor);
                    }
                }
                _ => {
                    if has_sensor {
                        commands.entity(head).remove::<Sensor>();
                    }
                }
            }
        }
    }
}

fn handle_grounded(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&ShapeHits, &mut PlayerState, Has<HeadBlocked>)>,
) {
    if let Ok((hits, mut player_state, has_headblocked)) = player.get_single_mut() {
        if !hits.is_empty() {
            if input.pressed(KeyCode::KeyQ) {
                *player_state = PlayerState::Crouching;
            } else {
                if !has_headblocked {
                    *player_state = PlayerState::Standing;
                } else {
                    *player_state = PlayerState::Crouching;
                }
            }
        } else {
            *player_state = PlayerState::Airborne;
        }
    }
}
