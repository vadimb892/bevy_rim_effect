use bevy::input::mouse::MouseWheel;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::ecs::schedule::SystemSet;

use outlines::outline::{Outline, OutlineLabel};
use outlines::rim_effect::RimEffect;

pub struct OutlinesPlugin;

impl Plugin for OutlinesPlugin 
{
    fn build(&self, app: &mut App)
    {
        add_outline::< RimEffect >( app );
    }
}

#[ derive( SystemSet, Debug, Hash, PartialEq, Eq, Clone ) ]
pub struct OutlineInitSet;

fn add_outline< O : OutlineLabel >( app : &mut App )
where 
    Outline< O > : MaterialExtension,
    MaterialPlugin::< ExtendedMaterial< StandardMaterial, Outline< O > > >: Plugin
{
    O::load_shader(app);

    app.add_plugins(
            MaterialPlugin::< ExtendedMaterial< StandardMaterial, Outline< O > > >::default( )
        )
        .register_type::< Outline< O > >()
        .add_systems( Update, ( 
            change_time_scale::< O >,
            change_outline_width::< O >,
            set_mode::< O >,
            update_material_time::< O >  
        ));
}


fn update_material_time< O : OutlineLabel >(
    time : Res< Time >,
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
)
where Outline< O > : MaterialExtension{
    for ( _, material ) in materials.iter_mut( ) {
        material.extension.add_time( material.extension.time_scale * time.delta_seconds( ) );
    }
}

fn change_outline_width< O : OutlineLabel >(
    keyboard_input : Res< ButtonInput< KeyCode > >, 
    mut mouse_wheel_events: EventReader< MouseWheel >,
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
    query: Query<&Handle<ExtendedMaterial< StandardMaterial, Outline< O > >>>
)
where Outline< O > : MaterialExtension{
    if keyboard_input.pressed( KeyCode::KeyW ){
        for event in mouse_wheel_events.read( ){
            for handle in query.iter(){
                if let Some(material) = materials.get_mut(handle){
                    if event.y > 0.0 {
                        material.extension.width *= 1.25;
                    }else {
                        material.extension.width /= 1.25;
                    }
                    info!( "{}", material.extension.width );
                }
            }
        }

    }
}

fn set_mode< O : OutlineLabel >(   
    keyboard_input : Res< ButtonInput< KeyCode > >,
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O > > > >,
    query: Query<&Handle<ExtendedMaterial< StandardMaterial, Outline< O > >>> 
)
where Outline< O > : MaterialExtension
{
    if keyboard_input.pressed( KeyCode::KeyR ){
        for handle in query.iter(){
            if let Some(material) = materials.get_mut(handle){
                let last = material.extension.is_time_related;
                material.extension.is_time_related = ( !( last > 0 ) ) as u32;
            }
        }
    }
}

pub fn change_time_scale< O : OutlineLabel >(
    keyboard_input : Res< ButtonInput< KeyCode > >, 
    mut mouse_wheel_events: EventReader< MouseWheel >,
    mut materials : ResMut< Assets< ExtendedMaterial< StandardMaterial, Outline< O >>>>,
    query: Query<&Handle<ExtendedMaterial< StandardMaterial, Outline< O > >>> 
)
where Outline< O > : MaterialExtension
{
    if keyboard_input.pressed( KeyCode::KeyT )
    {
        for event in mouse_wheel_events.read( )
        {
            for handle in query.iter(){
                if let Some(material) = materials.get_mut(handle){
                    if event.y > 0.0 {
                        material.extension.time_scale *= 1.25;
                    }
                    else {
                        material.extension.time_scale /= 1.25;
                    }
                    info!( "{}", material.extension.time_scale );
                }
            }
        }
    }
}