use bevy::{
    prelude::*,
    render::texture::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
};

/// Contain loaded [`Image`] assets
#[derive(Resource, Debug, Default)]
pub struct TextureAssets {
    /// [`TextureSet`] for ground [`StandardMaterial`]
    pub prototype: TextureSet,
    /// [`TextureSet`] for ceramic [`StandardMaterial`]
    pub ceramic: TextureSet,
}

/// Contain loaded [`Mesh`] assets
#[derive(Resource, Debug, Default)]
pub struct MeshAssets {
    /// Pyramida [`Mesh`]
    pub pyramida: Option<Handle<Mesh>>,
}

/// Loads assets for shapes [`StandardMaterial`]
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextureAssets>()
            .init_resource::<MeshAssets>()
            .add_systems(Startup, load_assets.in_set(AssetsInitSet));
    }
}

/// Used for ordering asset load systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AssetsInitSet;

/// Load assets for [`TextureAssets`], [`MeshAssets`]
fn load_assets(
    mut texture_assets: ResMut<TextureAssets>,
    mut mesh_assets: ResMut<MeshAssets>,
    asset_server: Res<AssetServer>,
) {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        min_filter: ImageFilterMode::Nearest,
        mag_filter: ImageFilterMode::Nearest,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    *texture_assets = TextureAssets {
        prototype: TextureSet {
            base: Some(asset_server.load_with_settings("textures\\texture_02.png", settings)),
            ..default()
        },
        ceramic: TextureSet {
            base: Some(asset_server.load(
                "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_COL_1K.jpg",
            )),
            normal: Some(asset_server.load(
                "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_NRM_1K.jpg",
            )),
            metallic: Some(asset_server.load(
                "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_REFL_1K.jpg",
            )),
            occlusion: Some(asset_server.load(
                "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_AO_1K.jpg",
            )),
            depth: Some(asset_server.load(
                "textures\\TilesCeramicHerringbone002\\TilesCeramicHerringbone002_DISP_1K.jpg",
            )),
            ..default()
        },
    };

    *mesh_assets = MeshAssets {
        pyramida: Some(asset_server.load("scenes\\pyramida.glb#Mesh0/Primitive0")),
    };
}

/// [`Image`] set for filling [`StandardMaterial`]
#[derive(Debug, Default)]
pub struct TextureSet {
    pub base: Option<Handle<Image>>,
    pub normal: Option<Handle<Image>>,
    pub metallic: Option<Handle<Image>>,
    pub occlusion: Option<Handle<Image>>,
    pub depth: Option<Handle<Image>>,
}
