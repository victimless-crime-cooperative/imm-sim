use avian3d::prelude::*;
use bevy::prelude::*;
use imm_sim_shared::{
    ownership::OwnedByClient,
    physics::components::movement::{JumpImpulse, LateralDamping, MovementAcceleration, SlopeData},
    physics::components::transform::ReplicatedTransform,
    player::components::PlayerAvatarColor,
};

use crate::camera::{CameraConfig, OwnedCamera};
use crate::connect::{ClientId, ConnectionState};

mod collision;

/// At present this plugin will manage any client-side state for the [`Player`]-related entities.
///
/// This includes:
///   1. Spawning the [`Mesh3d`] and [`MeshMaterial3d`] when a new player joins.
///   2. Updating the [`MeshMaterial3d`] when a [`PlayerAvatarColor`] changes.
pub struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_player_mesh, mutate_player_color).run_if(in_state(ConnectionState::InGame)),
        );
    }
}

/// The active client's "owned" player entity is marked with this tag.
#[derive(Component, Debug)]
pub struct OwnedPlayer;

/// Spawn the [`Mesh3d`] and [`MeshMaterial3d`] for a given player entity that does not currently
/// have one.
fn spawn_player_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    this_client: Res<ClientId>,

    query: Query<
        (
            &PlayerAvatarColor,
            &ReplicatedTransform,
            &OwnedByClient,
            Entity,
        ),
        Without<Mesh3d>,
    >,

    mut commands: Commands,
) {
    for (color, transform, owned_by, entity) in query.iter() {
        let mesh = meshes.add(Capsule3d::new(0.3, 2.0));
        let material = materials.add(StandardMaterial::from_color(color.0));

        let (shape_caster, player_top, player_bottom) =
            collision::generate_collision_components(1.0);

        let collision_layers = collision::generate_collision_layers();

        let mut cmd = commands.entity(entity);
        cmd.insert((
            // Debug mesh and material
            Mesh3d(mesh),
            MeshMaterial3d(material),
            RigidBody::Dynamic,
            Transform::from_translation(transform.translation).rotate(transform.rotation),
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

        if this_client.0 == owned_by.client_id {
            cmd.insert(OwnedPlayer);

            commands.spawn((Camera3d::default(), Transform::default(), OwnedCamera));
            commands.insert_resource(CameraConfig::default());
        }
    }
}

fn mutate_player_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (&PlayerAvatarColor, &mut MeshMaterial3d<StandardMaterial>),
        Changed<PlayerAvatarColor>,
    >,
) {
    for (color, mut material) in query.iter_mut() {
        let new_material = materials.add(StandardMaterial::from_color(color.0));
        material.0 = new_material;
    }
}
