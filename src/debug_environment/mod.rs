use bevy::prelude::*;

pub struct Block {
    translation: Vec3,
    extents: Vec3,
    color: Color,
}

impl Block {
    pub fn new(translation: Vec3, extents: Vec3, color: Color) -> Self {
        Self {
            translation,
            extents,
            color,
        }
    }
}

impl Command for Block {
    fn apply(self, world: &mut World) {
        let Vec3 { x, y, z } = self.extents;
        let mesh_handle = world
            .resource_mut::<Assets<Mesh>>()
            .add(Cuboid::new(x, y, z));
        let material_handle = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(self.color);

        world.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle),
            Transform::from_translation(self.translation),
        ));
    }
}
