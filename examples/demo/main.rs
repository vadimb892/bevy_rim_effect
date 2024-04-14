mod asset_loader;
mod camera;
mod light;
mod outlines;
mod world;
mod ui_help;

use bevy::prelude::*;

use bevy_atmosphere::prelude::*;

use asset_loader::{AssetLoaderPlugin, AssetsInitSet};
use camera::{CameraInitSet, CameraPlugin};
use light::LightPlugin;
use outlines::{OutlineInitSet, OutlinesPlugin};
use world::{SpawnInitSet, WorldPlugin};
use ui_help::UiHelpPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AtmospherePlugin
        ))
        .add_plugins((
            AssetLoaderPlugin,
            OutlinesPlugin,
            WorldPlugin,
            CameraPlugin,
            LightPlugin,
            UiHelpPlugin
        ))
        .configure_sets(
            Startup,
            (AssetsInitSet, CameraInitSet, OutlineInitSet, SpawnInitSet).chain(),
        )
        .run();
}
