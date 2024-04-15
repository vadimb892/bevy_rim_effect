use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::view::ColorGrading,
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use defaults::Defaults;
use std::f32::consts::PI;

/// [`Camera`] distance from view center
const CAMERA_DISTANCE: f32 = 20.0;
/// [`Camera`] focus coords
const FOCUS: Vec3 = Vec3::new(0.0, 2.0, 0.0);
/// [`Camera`] rotation speed
const CAMERA_ROTATION_SPEED: f32 = 15.0;

/// Label [`Camera`] as main
#[derive(Component, Debug)]
pub struct MainCamera;

/// Adds [`Camera`] movement behavior
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera.in_set(CameraInitSet))
            .add_systems(Update, camera_movement);
    }
}

/// Used for ordering camera behavior systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraInitSet;

/// Bind [`AtmosphereCamera`], [`UpHemisphereTransform`] to [`Camera`] entity and spawn it 
fn spawn_camera(
    mut commands: Commands,
) {
    let transform = Transform::from_translation(Vec3::new(CAMERA_DISTANCE, 0.0, 0.0) + FOCUS);
    commands.spawn((
        Camera3dBundle {
            transform,
            color_grading: ColorGrading {
                exposure: 0.0,
                gamma: 1.0,
                pre_saturation: 1.0,
                post_saturation: 1.0,
            },
            tonemapping: Tonemapping::ReinhardLuminance,
            ..default()
        },
        AtmosphereCamera::default(),
        UpHemisphereTransform::default(),
        MainCamera,
    ));
}

/// Contain [`Camera`] angles, radius, focus
#[derive(Component, Defaults)]
struct UpHemisphereTransform {
    /// [`Camera`] yaw angle 
    #[def = "45.0"]
    yaw: f32,
    /// [`Camera`] pitch angle
    #[def = "45.0"]
    pitch: f32,
    /// [`Camera`] distance from view center
    #[def = "CAMERA_DISTANCE"]
    radius: f32,
    /// [`Camera`] focus coords
    #[def = "FOCUS"]
    focus: Vec3,
}

impl UpHemisphereTransform {
    /// Updates [`UpHemisphereTransform`]
    fn add(&mut self, r: f32, yaw: f32, pitch: f32) -> Vec3 {
        self.radius *= r;
        self.yaw += yaw;
        self.pitch += pitch;

        if self.radius <= 1.0 {
            self.radius = 1.1;
        } else if self.radius >= 100.0 {
            self.radius = 99.9;
        }
        if self.yaw > 359.0 {
            self.yaw = 0.0;
        }
        if self.pitch <= 0.0 {
            self.pitch = 1.0;
        } else if self.pitch >= 90.0 {
            self.pitch = 89.0
        }
        self.get()
    }

    /// Returns [`Camera`] transition
    fn get(&self) -> Vec3 {
        let t = sphere_to_decart(Vec3::new(self.radius, self.pitch, self.yaw));
        self.focus + t
    }

    /// Changes [`Camera`] focus
    fn _set_focus(&mut self, focus: Vec3) -> Vec3 {
        self.focus = focus;
        self.get()
    }
}

/// (r, theta, phi) -> (x, y, z)
fn sphere_to_decart(sphere_coords: Vec3) -> Vec3 {
    let Vec3 {
        x: r,
        y: mut theta,
        z: mut phi,
    } = sphere_coords;
    theta = (theta / 180.0) * PI;
    phi = (phi / 180.0) * PI;
    Vec3::new(
        r * theta.sin() * phi.cos(),
        r * theta.cos(),
        r * theta.sin() * phi.sin(),
    )
}

/// Updates [`Camera`] [`UpHemisphereTransform`] from [`MouseWheel`], [`MouseMotion`]
fn camera_movement(
    mut query: Query<(&mut Transform, &mut UpHemisphereTransform), With<MainCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut orbital)) = query.get_single_mut() else {
        return;
    };
    let mut radius_mul = 1.0;
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    if !keyboard_input.any_pressed([KeyCode::KeyT, KeyCode::KeyW, KeyCode::KeyR]) {
        for event in mouse_wheel_events.read() {
            if event.y > 0.0 {
                radius_mul *= 1.25;
            } else {
                radius_mul /= 1.25;
            }
        }
    }

    for ev in mouse_motion.read() {
        yaw += CAMERA_ROTATION_SPEED * time.delta_seconds() * ev.delta.x;
        pitch += CAMERA_ROTATION_SPEED * time.delta_seconds() * ev.delta.y;
    }
    orbital.add(radius_mul, yaw, pitch);
    transform.translation = orbital.get();
    *transform = transform.looking_at(orbital.focus, Vec3::Y);
}
