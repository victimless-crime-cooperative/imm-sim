use avian3d::prelude::*;
use bevy::prelude::*;

pub struct DebugEnvironmentPlugin;

impl Plugin for DebugEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

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
            RigidBody::Static,
            Collider::cuboid(self.extents.x, self.extents.y, self.extents.z),
        ));
    }
}

pub fn setup(mut commands: Commands) {
    commands.queue(Block::new(
        Vec3::X * 2.0,
        Vec3::ONE,
        Color::srgb(1.0, 0.0, 0.0),
    ));
    commands.queue(Block::new(
        Vec3::NEG_X * 2.0,
        Vec3::ONE,
        Color::srgb(1.0, 0.0, 0.0),
    ));
    commands.queue(Block::new(
        Vec3::Z * 2.0,
        Vec3::ONE,
        Color::srgb(0.0, 1.0, 0.0),
    ));
    commands.queue(Block::new(
        Vec3::NEG_Y * 0.5,
        Vec3::new(5.0, 0.5, 5.0),
        Color::WHITE,
    ));
}
