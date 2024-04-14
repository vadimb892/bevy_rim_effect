use bevy::{asset::LoadState, prelude::*, render::{render_resource::{TextureViewDescriptor, TextureViewDimension}, texture::{ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor}}};


#[ derive( Resource, Debug, Default ) ]
pub struct TextureAssets 
{
  pub skybox : Handle< Image >,
  pub prototype : TextureSet,
  pub ceramic : TextureSet,
}

#[ derive( Resource, Debug, Default ) ]
pub struct MeshAssets
{
  pub cube_chest : Option< Handle< Mesh > >,
  pub pyramida : Option< Handle< Mesh > >
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin 
{
  fn build( &self, app: &mut App ) 
  {
    app.init_resource::< TextureAssets >( )
    .init_resource::< MeshAssets >( )
    .add_systems( Startup, ( load_assets, skybox_reinterpret ).chain( ).in_set( AssetsInitSet ) );
  }
}

#[ derive( SystemSet, Debug, Hash, PartialEq, Eq, Clone ) ]
pub struct AssetsInitSet;

fn load_assets(
  mut texture_assets: ResMut< TextureAssets >, 
  mut mesh_assets: ResMut< MeshAssets >, 
  asset_server: Res< AssetServer > 
) 
{
  let sampler_desc = ImageSamplerDescriptor {
    address_mode_u : ImageAddressMode::Repeat,
    address_mode_v : ImageAddressMode::Repeat,
    min_filter : ImageFilterMode::Nearest,
    mag_filter: ImageFilterMode::Nearest,
    ..Default::default( )
  };

  let settings = move | s : &mut ImageLoaderSettings | {
      s.sampler = ImageSampler::Descriptor( sampler_desc.clone( ) );
  };

  *texture_assets = TextureAssets
  {
    skybox : asset_server.load( "textures\\skybox\\skybox.png" ),
    prototype: TextureSet
    {
      base : Some( asset_server.load_with_settings( "textures\\texture_02.png", settings ) ),
      ..default( )
    },
    ceramic : TextureSet
    { 
      base : Some( asset_server.load( "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_COL_1K.jpg" ) ),
      normal : Some( asset_server.load( "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_NRM_1K.jpg" ) ),
      metallic : Some( asset_server.load( "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_REFL_1K.jpg" ) ),
      occlusion : Some( asset_server.load( "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_AO_1K.jpg" ) ),
      depth : Some( asset_server.load( "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_DISP_1K.jpg") ),
      ..default( )
    },
  };

  *mesh_assets = MeshAssets
  {
    cube_chest : Some( asset_server.load( "scenes\\cube_chest.glb#Mesh0/Primitive0" ) ),
    pyramida : Some( asset_server.load( "scenes\\pyramida.glb#Mesh0/Primitive0" ) )
  };
}

fn skybox_reinterpret( 
  mut images: ResMut< Assets< Image > >,
  texture_assets: ResMut< TextureAssets >, 
  asset_server: Res< AssetServer > 
)
{
  if asset_server.load_state(&texture_assets.skybox ) == LoadState::Loaded 
  {
    info!( "{:?}", asset_server.load_state(&texture_assets.skybox ) );
    let image = images.get_mut( &texture_assets.skybox ).unwrap( );
    if image.texture_descriptor.array_layer_count( ) == 1 {
      info!( "{}", image.texture_descriptor.array_layer_count( ) );
      image.reinterpret_stacked_2d_as_array( image.height( ) / image.width( ) );
      image.texture_view_descriptor = Some( TextureViewDescriptor {
          dimension: Some( TextureViewDimension::Cube ),
          ..default( )
      } );
    }
  }
}

#[ derive( Debug, Default ) ]
pub struct TextureSet
{
  pub base : Option< Handle< Image > >,
  pub normal : Option< Handle< Image > >,
  pub metallic : Option< Handle< Image > >,
  pub occlusion: Option< Handle< Image > >,
  pub depth : Option< Handle< Image > >,
}