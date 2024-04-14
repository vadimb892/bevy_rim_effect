use bevy::prelude::*;
use bevy::ecs::schedule::SystemSet;

use crate::asset_loader::TextureAssets;
use outline

pub struct OutlinesPlugin;

impl Plugin for OutlinesPlugin 
{
    fn build(&self, app: &mut App)
    {
        app
            .init_resource::< TimeScale >( )
            .init_resource::< Width >( )
            .add_systems( Update, change_time_scale );

        add_outline::< RimEffect >( app );
    }
}

#[ derive( SystemSet, Debug, Hash, PartialEq, Eq, Clone ) ]
pub struct OutlineInitSet;

fn add_outline< O : OutlineLabel >( app : &mut App )
where Outline< O > : MaterialExtension
{
    let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();

    let shader = Shader::from_wgsl(include_str!(O::shader_path()));

    shaders.set_untracked(O::shader_handle(), shader);

    app.add_plugins(
            MaterialPlugin::< ExtendedMaterial< StandardMaterial, Outline< O > > >::default( )
        )
        .register_type::< Outline< O > >()
        .add_systems( Update, ( 
            setup_material_extention::< O >,
            update_material_time::< O >, 
            change_outline_width::< O >,
            set_mode::< O >,
            update_material_time::< O >  
        ) );
}

fn update_material_time< O : OutlineLabel >(
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
    time : Res< Time >,
    time_scale : Res< TimeScale >
)
where Outline< O > : MaterialExtension{
    for ( _, material ) in materials.iter_mut( ) 
    {
        material.extension.add_time( time_scale.0 * time.delta_seconds( ) );
    }
}

fn change_outline_width< O : OutlineLabel >(
    keyboard_input : Res< ButtonInput< KeyCode > >, 
    mut mouse_wheel_events: EventReader< MouseWheel >,
    mut width : ResMut< Width >,
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
)
where Outline< O > : MaterialExtension{
    if keyboard_input.pressed( KeyCode::KeyW ){
        for event in mouse_wheel_events.read( ){
            if event.y > 0.0 
            {
            width.0 *= 1.25;
            }
            else 
            {
            width.0 /= 1.25;
            }
            info!( "{}", width.0 );
        }
        for ( _, material ) in materials.iter_mut( ) {
            material.extension.width = width.0;
        }
    }
}

fn set_mode< O : OutlineLabel >(   
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
    keyboard_input : Res< ButtonInput< KeyCode > >, 
)
where Outline< O > : MaterialExtension
{
    if keyboard_input.pressed( KeyCode::KeyR )
    {
    for ( _, material ) in materials.iter_mut( ) 
    {
        let last = material.extension.is_time_related;
        material.extension.is_time_related = ( !( last > 0 ) ) as u32;
    }
    }
}

fn setup_material_extention< O : OutlineLabel >(
    mut commands : Commands,
    texture_assets: Res< TextureAssets >,
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
)
where Outline< O > : MaterialExtension{
let t = &texture_assets.ceramic;
    commands.spawn( (
        materials.add( 
            ExtendedMaterial {
                base: StandardMaterial { 
                    base_color_texture: t.base.clone( ),
                    normal_map_texture : t.normal.clone( ),
                    metallic_roughness_texture : t.metallic.clone( ),
                    occlusion_texture : t.occlusion.clone( ),
                    alpha_mode : AlphaMode::Blend,
                    ..default()
                },
                extension : Outline::< O >::default( )
            } ),
        O::default( )
    ) );
}

#[ derive( Resource, Defaults ) ]
pub struct TimeScale
(
    #[def = "1.0"]
    pub f32
);

pub fn change_time_scale
(
    keyboard_input : Res< ButtonInput< KeyCode > >, 
    mut mouse_wheel_events: EventReader< MouseWheel >,
    mut time_scale : ResMut< TimeScale >
)
{
    if keyboard_input.pressed( KeyCode::KeyT )
    {
        for event in mouse_wheel_events.read( )
        {
            if event.y > 0.0 
            {
            time_scale.0 *= 1.25;
            }
            else 
            {
            time_scale.0 /= 1.25;
            }
            info!( "{}", time_scale.0 );
        }
    }
}

const WIDTH : f32 = 5.0;

#[ derive( Resource, Defaults ) ]
pub struct Width
(
    #[def = "WIDTH"]
    pub f32
);