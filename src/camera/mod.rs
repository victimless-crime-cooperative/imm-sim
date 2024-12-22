use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (read_rotation_inputs, position_camera));
    }
}

pub struct CameraSensitivity {
    pub x: f32,
    pub y: f32,
}

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

#[derive(Resource)]
pub struct CameraConfig {
    smoothing: f32,
    sensitivity: CameraSensitivity,
    x_angle: f32,
    y_angle: f32,
    x_limits: (f32, f32),
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            smoothing: 5.0,
            sensitivity: CameraSensitivity::default(),
            x_angle: 0.0,
            y_angle: 0.0,
            x_limits: (-85.0, 90.0),
        }
    }
}

impl CameraConfig {
    pub fn rotate(&mut self, x: f32, y: f32) {
        self.x_angle = (x + self.x_angle).clamp(self.x_limits.0, self.x_limits.1);
        self.y_angle += y;
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, self.y_angle.to_radians())
            * Quat::from_axis_angle(Vec3::X, self.x_angle.to_radians())
    }
}

fn read_rotation_inputs(
    time: Res<Time>,
    mut camera_config: ResMut<CameraConfig>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if mouse_motion.delta != Vec2::ZERO {
        let Vec2 { x, y } = mouse_motion.delta;
        let x_rotation = time.delta_secs() * camera_config.sensitivity.y * 180.0 * -y;
        let y_rotation = time.delta_secs() * camera_config.sensitivity.x * 180.0 * x;
        camera_config.rotate(x_rotation, y_rotation);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::Y),
        MainCamera,
    ));

    use crate::debug_environment::Block;

    commands.queue(Block::new(Vec3::X, Vec3::ONE, Color::srgb(1.0, 0.0, 0.0)));
    commands.queue(Block::new(
        Vec3::NEG_X,
        Vec3::ONE,
        Color::srgb(1.0, 0.0, 0.0),
    ));
}

fn position_camera(
    time: Res<Time>,
    camera_config: Res<CameraConfig>,
    camera: Single<&mut Transform, With<MainCamera>>,
) {
    let mut camera_transform = camera.into_inner();
    camera_transform.rotation.smooth_nudge(
        &camera_config.rotation(),
        camera_config.smoothing,
        time.delta_secs(),
    );
}
