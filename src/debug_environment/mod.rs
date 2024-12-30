use avian3d::prelude::*;
use bevy::prelude::*;

pub struct DebugEnvironmentPlugin;

impl Plugin for DebugEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, draw_rulers);
    }
}

pub struct Block {
    translation: Vec3,
    extents: Vec3,
    color: Color,
}

#[derive(Component)]
struct Ruler(usize, Dir3, Dir3);

fn draw_rulers(mut gizmos: Gizmos, query: Query<(&Ruler, &Transform)>) {
    for (ruler, transform) in &query {
        let unit: Vec3 = ruler.1.into();
        let perpendicular_unit: Vec3 = ruler.2.into();
        let origin: Vec3 = transform.translation;

        for i in 0..ruler.0 {
            let point = origin + unit * i as f32;
            let half_point = point - (unit * 0.5);

            gizmos.line(
                point - perpendicular_unit,
                point + perpendicular_unit,
                Srgba::hex("#FF0000").unwrap(),
            );

            gizmos.line(
                half_point - (perpendicular_unit * 0.5),
                half_point + (perpendicular_unit * 0.5),
                Srgba::hex("#FF0000").unwrap(),
            );
        }
    }
}

pub struct GizmoRuler {
    origin: Vec3,
    length: usize,
    direction: Dir3,
    cross: Dir3,
}

impl GizmoRuler {
    pub fn new(origin: Vec3, length: usize, direction: Dir3, cross: Dir3) -> Self {
        Self {
            origin,
            length,
            direction,
            cross,
        }
    }
}

impl Command for GizmoRuler {
    fn apply(self, world: &mut World) {
        world.spawn((
            Transform::from_translation(self.origin),
            Ruler(self.length, self.direction, self.cross),
        ));
    }
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
        Vec3::new(2.0, 0.5, 0.0),
        Vec3::ONE,
        Color::srgb(1.0, 0.0, 0.0),
    ));
    commands.queue(Block::new(
        Vec3::new(-2.0, 0.5, 0.0),
        Vec3::ONE,
        Color::srgb(1.0, 0.0, 0.0),
    ));
    commands.queue(Block::new(
        Vec3::new(0.0, 0.5, 2.0),
        Vec3::ONE,
        Color::srgb(0.0, 1.0, 0.0),
    ));

    // Half height barrier
    commands.queue(Block::new(
        Vec3::new(2.0, 0.75, 2.0),
        Vec3::new(1.0, 0.5, 1.0),
        Color::srgb(0.0, 1.0, 0.0),
    ));
    commands.queue(Block::new(
        Vec3::NEG_Y * 0.25,
        Vec3::new(20.0, 0.5, 20.0),
        Color::WHITE,
    ));

    commands.queue(GizmoRuler::new(
        Vec3::new(0.0, 0.0, -2.0),
        10,
        Dir3::Y,
        Dir3::X,
    ));
}
