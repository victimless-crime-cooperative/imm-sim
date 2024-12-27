use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::{client::ClientSet, prelude::*};
use imm_sim_shared::{
    ownership::OwnedByClient, physics::components::transform::ReplicatedTransform,
    player::messages::server_commands::S2CSpawnPlayerCommand,
};

use crate::connect::ConnectionState;

pub struct MirrorStatePlugin;

impl Plugin for MirrorStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            mirror_replicated_transform
                .run_if(in_state(ConnectionState::InGame))
                .after(ClientSet::Receive),
        )
        .add_systems(
            Update,
            spawn_player.run_if(in_state(ConnectionState::InGame)),
        );
    }
}

pub fn spawn_player(
    mut reader: EventReader<S2CSpawnPlayerCommand>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    query: Query<(&OwnedByClient, Entity)>,
    mut commands: Commands,
) {
    for cmd in reader.read() {
        let S2CSpawnPlayerCommand {
            for_client_id,
            initial_translation,
            initial_rotation,
            initial_color,
            ..
        } = cmd;

        for (owned_by, entity) in query.iter() {
            if owned_by.client_id == *for_client_id {
                let mesh = meshes.add(Capsule3d::new(0.3, 2.0));
                let material = materials.add(StandardMaterial::from_color(*initial_color));

                commands.entity(entity).insert((
                    Mesh3d(mesh),
                    MeshMaterial3d(material),
                    RigidBody::Dynamic,
                    Transform::from_translation(*initial_translation).rotate(*initial_rotation),
                    LockedAxes::new().lock_rotation_x().lock_rotation_z(),
                    ShapeCaster::new(
                        Collider::capsule(0.3, 2.0),
                        Vec3::ZERO,
                        Quat::IDENTITY,
                        Dir3::NEG_Y,
                    )
                    .with_ignore_self(true)
                    .with_max_distance(1.0),
                ));
            }
        }
    }
}

pub fn mirror_replicated_transform(mut query: Query<(&ReplicatedTransform, &mut Transform)>) {
    for (replica, mut transform) in query.iter_mut() {
        *transform = (*replica).into();
    }
}
