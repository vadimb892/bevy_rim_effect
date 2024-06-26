#import bevy_pbr::{
  pbr_fragment::pbr_input_from_standard_material,
  pbr_types::PbrInput,
  forward_io::{VertexOutput,FragmentOutput},
  pbr_functions::{
    alpha_discard,
    apply_pbr_lighting,
    main_pass_post_lighting_processing
  }
}

// Value that helps change animation and synchonize it with other entities outline material
@group(2) @binding(100)
var<uniform> u_time : f32;
// Outline width (power of Fresnel saturation for rim effect)
@group(2) @binding(101)
var<uniform> width : f32;
// Set width change state: (const width)/(time depended)
@group(2) @binding(102)
var<uniform> is_time_related : u32;

// Use normal, view vector for rim effect outline level calculation
fn rim_effect( pbr_input : PbrInput ) -> f32
{
  var power : f32 = width; 
  if 0 < is_time_related{
    power =  sin(u_time) * 0.6 + width;
  }
  var N : vec3f = pbr_input.N;
  let V : vec3f = pbr_input.V;
  var fresnel =  1.0 - max( dot( N, V ), 0.0001 );
  fresnel = 2.0 * pow( saturate( fresnel ), power );
  return fresnel;
}

// Material extension fragment shader from: https://github.com/bevyengine/bevy/blob/main/assets/shaders/extended_material.wgsl
// But there also added `rim_effect` call that mixed with `out.color` 
@fragment
fn fragment( 
  in: VertexOutput,
  @builtin(front_facing) is_front: bool,
) -> FragmentOutput
{
  var pbr_input = pbr_input_from_standard_material( in, is_front );
  pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);
  var uv : vec2f = in.uv;
  var out: FragmentOutput;
  out.color = apply_pbr_lighting(pbr_input);
  out.color = main_pass_post_lighting_processing(pbr_input, out.color);
  let outline_color : vec4f = vec4f( 0.6, 0.6, 0.0, 1.0 );
  out.color = mix( out.color, outline_color, rim_effect( pbr_input ) );
  return out;
}
