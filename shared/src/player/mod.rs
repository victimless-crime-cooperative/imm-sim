use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::Replicated;

use crate::{ownership::OwnedByClient, physics::components::transform::ReplicatedTransform};

use self::{
    components::{PlayerAvatarColor, PlayerDisplayName},
    messages::server_commands::S2CSpawnPlayerCommand,
};

pub mod components;
pub mod messages;

pub trait SpawnPlayerCommandsExt {
    fn spawn_player_server(&mut self, cmd: &S2CSpawnPlayerCommand) -> EntityCommands<'_>;
}

impl<'w, 's> SpawnPlayerCommandsExt for Commands<'w, 's> {
    fn spawn_player_server(&mut self, cmd: &S2CSpawnPlayerCommand) -> EntityCommands<'_> {
        let S2CSpawnPlayerCommand {
            for_client_id,
            display_name,
            initial_translation,
            initial_rotation,
            initial_color,
        } = cmd;

        // All the replicaetd components
        let mut cmd = self.spawn((
            Replicated,
            OwnedByClient {
                client_id: *for_client_id,
            },
            PlayerAvatarColor(*initial_color),
            PlayerDisplayName(display_name.clone()),
            ReplicatedTransform {
                translation: *initial_translation,
                rotation: *initial_rotation,
                scale: Vec3::ONE,
            },
        ));
        // Then all the local physics components
        cmd.insert((
            RigidBody::Dynamic,
            Transform::from_translation(*initial_translation).rotate(*initial_rotation),
            LockedAxes::new().lock_rotation_x().lock_translation_z(),
            ShapeCaster::new(
                Collider::capsule(0.3, 2.0),
                Vec3::ZERO,
                Quat::IDENTITY,
                Dir3::NEG_Y,
            )
            .with_ignore_self(true)
            .with_max_distance(1.0),
        ));

        cmd
    }
}
