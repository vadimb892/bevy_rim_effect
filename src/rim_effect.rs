use bevy::prelude::*;
use bevy::pbr::MaterialExtension;
use bevy::render::render_resource::ShaderRef;

use super::outline::{Outline, OutlineLabel};

pub const SHADER_HANDLE: Handle::<Shader> = 
    Handle::<Shader>::weak_from_u128(42572727525244273574);

#[ derive( Reflect, Component, Clone, Default ) ]
pub struct RimEffect;

impl OutlineLabel for RimEffect {
    fn shader_handle() -> Handle::<Shader>{
        SHADER_HANDLE
    }
    fn shader_path() -> &'static str{
        "shaders/fragment.wgsl"
    }
}

impl MaterialExtension for Outline< RimEffect > 
{
    fn fragment_shader( ) -> ShaderRef 
    {
        SHADER_HANDLE.into()
    }
}