use std::f32::consts::PI;

use crate::asset_loader::{MeshAssets, TextureAssets};
use bevy::{pbr::ExtendedMaterial, prelude::*, render::mesh::VertexAttributeValues};
use outlines::{outline::Outline, rim_effect::RimEffect};

/// Setup entities for [`App`]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
                init_shapes_settings,
                map_shapes_transform,
                spawn_shapes,
                spawn_plane
            ).chain()
            .in_set(SpawnInitSet)
        );
    }
}

/// Used for ordering shape spawn systems
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnInitSet;

/// One shape settings that used in entity spawn
#[derive(Component, Clone, Debug)]
pub struct Shape{
    pub transform: Transform, 
    pub shape: Handle< Mesh >,
}

/// Colection of [`Shapes`] that must be spawn in [`Startup`]
#[derive(Clone, Debug, Default, Resource)]
pub struct Shapes(pub Vec<Shape>);

/// Setup [`Shapes`]
fn init_shapes_settings(
    mut commands: Commands,
    mut meshes : ResMut< Assets< Mesh > >,
    mesh_assets: Res< MeshAssets >,
){
    let q = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
    let v = Vec3::new(1.0, 1.0, 1.0);

    commands.insert_resource(Shapes(Vec::from([
        //Cube
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 1.75, 0.0),
                rotation: q.clone(),
                scale: 3.5 * v.clone(),
            },
            shape: meshes.add(Cuboid::default()),
        },
        //Sphere
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 2.5, 0.0),
                rotation: q.clone(),
                scale: 5.0 * v.clone(),
            },
            shape: meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        },
        //Capsule
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 3.0, 0.0),
                rotation: Quat::from_euler(EulerRot::XYZ, 45.0, 45.0, 0.0),
                scale: 4.0 * v.clone(),
            },
            shape: meshes.add(Capsule3d::default()),
        },
        //Cylinder
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 1.77, 0.0),
                rotation: q.clone(),
                scale: 3.5 * v.clone(),
            },
            shape: meshes.add(Cylinder::default()),
        },
        //Cuboid
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: q.clone(),
                scale: Vec3::new(3.0, 5.0, 3.0),
            },
            shape: meshes.add(Cuboid::default()),
        },
        //Tetraedron
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 1.65, 0.0),
                rotation: q.clone(),
                scale: 3.5 * v.clone(),
            },
            shape: mesh_assets.pyramida.as_ref().unwrap().clone(),
        },
        //Character: Capsule + Sphere
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 1.5, 0.0),
                rotation: q.clone(),
                scale: 1.5 * v.clone(),
            },
            shape: meshes.add(Capsule3d::default()),
        },
        Shape {
            transform: Transform {
                translation: Vec3::new(0.0, 3.7, 0.0),
                rotation: q.clone(),
                scale: v.clone(),
            },
            shape: meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        },
    ])));
}

/// Changes [`Transform`] of some [`Shape`]s so that they stand in a circle
fn map_shapes_transform(
    mut shapes: ResMut<Shapes>
){
    let shapes_in_cirlce_count: usize = shapes.0.len() - 2;
    let degrees_step = 360.0 / shapes_in_cirlce_count as f32;
    for (i, shape) in shapes.0.iter_mut().enumerate(){
        if i < shapes_in_cirlce_count{
            let Vec2 { x, y: z } = polar_to_decart(Vec2::new(7.0, degrees_step * i as f32));
            let translation_y = shape.transform.translation.y;
            shape.transform.translation = Vec3::new(x, translation_y, z);
        }
    }
}

/// Spawn all [`Shape`] from [`Shapes`] resource list
fn spawn_shapes(
    mut commands: Commands,
    texture_assets: Res< TextureAssets >,
    mut materials: ResMut<Assets<ExtendedMaterial< StandardMaterial, Outline< RimEffect > >>>,
    shapes: Res<Shapes>
){
    let t = &texture_assets.ceramic;

    for shape in shapes.0.iter( )
    {
        let Shape{ transform, shape } = shape;
        commands.spawn(( 
            MaterialMeshBundle
            {
                mesh: shape.clone(),
                transform: *transform,
                material : materials.add( 
                    ExtendedMaterial {
                        base: StandardMaterial { 
                            base_color_texture: t.base.clone( ),
                            normal_map_texture : t.normal.clone( ),
                            metallic_roughness_texture : t.metallic.clone( ),
                            occlusion_texture : t.occlusion.clone( ),
                            alpha_mode : AlphaMode::Blend,
                            ..default()
                        },
                        extension : Outline::< RimEffect >::default( )
                    } ),    
                ..default( )
            },
            RimEffect::default( ),
        ));
    }
}

/// Spawn ground plane in [`Startup`]
fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_assets: Res<TextureAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mut mesh = Mesh::from(Plane3d::default()).rotated_by(Quat::from_rotation_y(-PI));

    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs {
            uv[0] *= 250.0;
            uv[1] *= 250.0;
        }
    };

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_scale(Vec3::new(1000.0, 1000.0, 1000.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.49, 0.6),
            base_color_texture: texture_assets.prototype.base.clone(),
            metallic: 0.0,
            perceptual_roughness: 0.1,
            ..default()
        }),
        ..default()
    });
}

fn polar_to_decart(polar_coords: Vec2) -> Vec2 {
    let Vec2 { x: r, y: mut theta } = polar_coords;
    theta = (theta / 180.0) * PI;
    Vec2::new(r * theta.cos(), r * theta.sin())
}