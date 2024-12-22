use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct PlayerHeadCollider;
#[derive(Component)]
pub struct PlayerFeetCollider;
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
            ))
            .with_children(|parent| {
                parent.spawn((
                    Transform::from_translation(Vec3::Y * self.height * 0.75),
                    PlayerHeadCollider,
                    Collider::sphere(0.25),
                ));
                parent.spawn((
                    Transform::from_translation(Vec3::Y * self.height * 0.25),
                    PlayerFeetCollider,
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
