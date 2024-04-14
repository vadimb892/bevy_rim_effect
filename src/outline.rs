use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;

pub trait OutlineLabel : Component + TypePath + FromReflect + Reflect + Clone + Default + Send + Sync { 
    fn shader_handle() -> Handle<Shader>;
    fn load_shader(app : &mut App);
}

#[derive( Asset, AsBindGroup, Reflect, Debug, Clone ) ]
pub struct Outline< O : OutlineLabel > 
{
    pub time_scale: f32,
    #[ uniform( 100 ) ]
    pub u_time : f32,
    #[ uniform( 101 ) ]
    pub width: f32,
    #[ uniform( 102 ) ]
    pub is_time_related : u32,
    _outline : O
}

impl< O : OutlineLabel > Outline< O >
{
    pub fn add_time( &mut self, delta_time : f32 )
    {
        self.u_time += delta_time;
    }
}

impl< O : OutlineLabel > Default for Outline< O >
{
    fn default( ) -> Outline< O >
    {
        Outline::< O >
        {
            time_scale: 1.0,
            u_time : 0.0,
            width : 2.0,
            is_time_related : 0,
            _outline : O::default( )
        }
    }
}

