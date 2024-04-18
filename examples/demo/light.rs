use bevy::{pbr::ScreenSpaceAmbientOcclusionBundle, prelude::*};
use std::f32::consts::PI;

/// Light setup
pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

/// Adds [`DirectionalLight`]
fn setup(mut commands: Commands) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 15000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 100.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle::default());
}
