use std::f32::consts::PI;
use bevy::{
  pbr::{light_consts, CascadeShadowConfigBuilder, ScreenSpaceAmbientOcclusionBundle}, prelude::*
};

pub struct LightPlugin;

impl Plugin for LightPlugin 
{
  fn build( &self, app: &mut App ) 
  {
    app.insert_resource( Msaa::Sample4 )
    .add_systems( Startup, setup );
  }
}

fn setup( mut commands : Commands )
{
  commands.spawn(DirectionalLightBundle 
    {
      directional_light : DirectionalLight {
        illuminance : light_consts::lux::AMBIENT_DAYLIGHT + 5000.0,
        shadows_enabled : true,
        ..default( )
      },
      transform : Transform 
      {
        translation : Vec3::new(100.0, 100.0, 100.0),
        rotation : Quat::from_rotation_x(-PI / 4.),
        ..default( )
      },
      ..default( )
    } )
  .insert( ScreenSpaceAmbientOcclusionBundle::default( ) );
}