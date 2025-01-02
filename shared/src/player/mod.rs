use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::Replicated;

use self::components::{Player, PlayerAvatarColor, PlayerDisplayName};
use crate::{ownership::OwnedByClient, physics::components::transform::ReplicatedTransform};

pub mod components;
pub mod messages;

#[cfg(feature = "server")]
pub trait SpawnPlayerCommandsExt {
    fn spawn_player(
        &mut self,
        client_id: u64,
        display_name: String,
        translation: Vec3,
        rotation: Quat,
        color: Color,
    ) -> EntityCommands<'_>;
}

#[cfg(feature = "server")]
impl<'w, 's> SpawnPlayerCommandsExt for Commands<'w, 's> {
    fn spawn_player(
        &mut self,
        client_id: u64,
        display_name: String,
        translation: Vec3,
        rotation: Quat,
        color: Color,
    ) -> EntityCommands<'_> {
        // All the replicated components
        let mut cmd = self.spawn((
            Replicated,
            Player,
            OwnedByClient { client_id },
            PlayerAvatarColor(color),
            PlayerDisplayName(display_name),
            ReplicatedTransform {
                translation,
                rotation,
                scale: Vec3::ONE,
            },
        ));

        // Then all the local physics components
        cmd.insert((
            RigidBody::Dynamic,
            Transform::from_translation(translation).rotate(rotation),
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
