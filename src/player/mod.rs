use crate::actions::{AirborneAction, StandingAction};
use crate::camera::CameraConfig;
use crate::physics::{Grounded, JumpImpulse, LateralDamping, MovementAcceleration, SlopeData};
use avian3d::prelude::*;
use bevy::prelude::*;

mod collision;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    read_grounded_movement_inputs,
                    read_airborne_movement_inputs,
                    handle_head_collider,
                    display_slope,
                ),
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
    Sliding,
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
                SlopeData::default(),
                shape_caster,
                collision_layers,
            ))
            .with_children(|parent| {
                parent.spawn(player_top);
                parent.spawn(player_bottom);
            });
    }
}

#[derive(Component)]
pub struct SlopeDisplay;

fn setup(mut commands: Commands) {
    commands.queue(SpawnPlayer {
        height: 1.0,
        translation: Vec3::NEG_Z + Vec3::Y * 30.0,
    });

    commands.spawn((SlopeDisplay, Text::new("Slope: ")));
}

fn display_slope(
    display_query: Single<&mut Text, With<SlopeDisplay>>,
    slope_query: Single<&SlopeData>,
) {
    let slope = slope_query.into_inner();
    let mut display = display_query.into_inner();
    display.0 = format!("Ground Normal: {}", slope.ground_normal);
}

fn read_grounded_movement_inputs(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<Entity, (With<Player>, With<Grounded>)>,
) {
    let player_entity = player.into_inner();
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let space = keyboard_input.pressed(KeyCode::Space);
    let crouch = keyboard_input.pressed(KeyCode::KeyQ);
    let uncrouch = !keyboard_input.pressed(KeyCode::KeyQ);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;

    let direction = camera_config.interpolate(Vec2::new(horizontal as f32, vertical as f32));

    if direction != Vec3::ZERO {
        commands.trigger_targets(StandingAction::Run(direction), player_entity);
    }

    if space {
        commands.trigger_targets(StandingAction::Jump, player_entity);
    }

    if crouch {
        commands.trigger_targets(StandingAction::Crouch(direction), player_entity);
    }

    if uncrouch {
        commands.trigger_targets(StandingAction::Uncrouch, player_entity);
    }
}

fn read_airborne_movement_inputs(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<Entity, (With<Player>, Without<Grounded>)>,
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
        commands.trigger_targets(AirborneAction::Move(direction), player_entity);
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
