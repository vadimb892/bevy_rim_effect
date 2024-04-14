mod asset_loader;
mod camera;
mod collider;
mod cursor;
mod hovering;
mod light;
mod outlines;
mod world;

use bevy::prelude::*;

use asset_loader::{AssetLoaderPlugin, AssetsInitSet};
use camera::{CameraInitSet, CameraPlugin};
use light::LightPlugin;
use outlines::{OutlineInitSet, OutlinesPlugin};
use world::{SpawnInitSet, WorldPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.39607, 0.7607, 0.9607)))
        .init_resource::<Hovering>()
        .add_plugins((
            DefaultPlugins,
            AssetLoaderPlugin,
            OutlinesPlugin,
            WorldPlugin,
            CameraPlugin,
            LightPlugin,
        ))
        .add_systems(
            Startup,
            setup_material_extention::<O>.in_set(super::OutlineInitSet),
        )
        .configure_sets(
            Startup,
            (AssetsInitSet, CameraInitSet, OutlineInitSet, SpawnInitSet).chain(),
        )
        .run();
}
