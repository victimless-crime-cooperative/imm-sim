use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Copy, Deserialize, PartialEq, Serialize)]
pub struct ReplicatedTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl From<Transform> for ReplicatedTransform {
    fn from(value: Transform) -> Self {
        let Transform {
            translation,
            rotation,
            scale,
        } = value;

        Self {
            translation,
            rotation,
            scale,
        }
    }
}

impl From<ReplicatedTransform> for Transform {
    fn from(value: ReplicatedTransform) -> Self {
        let ReplicatedTransform {
            translation,
            rotation,
            scale,
        } = value;

        Self {
            translation,
            rotation,
            scale,
        }
    }
}
