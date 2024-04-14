use std::f32::consts::PI;

use crate::{
    asset_loader::{MeshAssets, TextureAssets},
    outline::{self, RimEffect},
};
use bevy::{pbr::ExtendedMaterial, prelude::*, render::mesh::VertexAttributeValues};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_shapes,
            spawn_plane
            ).in_set(SpawnInitSet)
        );

        let q = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
        let v = Vec3::new(1.0, 1.0, 1.0);

        app.insert_resource(Shapes(Vec::from([
            //Cube
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 1.75, 0),
                    rotation: q.clone(),
                    scale: 3.5 * v.clone(),
                }),
                shape: meshes.add(Cuboid::default()),
                colliders: [DisplacedCollider::bundle(
                    Collider::cuboid(3.5, 3.5, 3.5),
                    Transform::default()
                )]
                .into(),
            },
            //Sphere
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 2.5, 0),
                    rotation: q.clone(),
                    scale: 5.0 * v.clone(),
                }),
                shape: meshes.add(Sphere::default().mesh().ico(5).unwrap()),
                colliders: [DisplacedCollider::bundle(
                    Collider::ball(2.5),
                    Transform::default()
                )]
                .into(),
            },
            //Torus
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 1.2, 0),
                    rotation: q.clone(),
                    scale: 1.5 * v.clone(),
                }),
                shape: meshes.add(Torus::new(1.0, 3.0)),
                colliders: [DisplacedCollider::bundle(
                    Collider::cylinder(1.2, 3.0),
                    Transform::default()
                )]
                .into(),
            },
            //Cylinder
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 1.77, 0),
                    rotation: q.clone(),
                    scale: 3.5 * v.clone(),
                }),
                shape: meshes.add(Cylinder::default()),
                colliders: [DisplacedCollider::bundle(
                    Collider::cylinder(1.77, 1),
                    Transform::default()
                )]
                .into(),
            },
            //Cubic cross
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 2.65, 0),
                    rotation: Quat::from_euler(EulerRot::XYZ, 48.0, 48.0, 0.0),
                    scale: 0.9 * v.clone(),
                }),
                shape: mesh_assets.cube_chest.as_ref().unwrap().clone(),
                colliders: [
                    DisplacedCollider::bundle(
                        Collider::cuboid(0.9, 0.9, 2.7),
                        Transform::from_rotation(
                            Quat::default(),
                        ),
                    ),
                    DisplacedCollider::bundle(
                        Collider::cuboid(0.9, 0.9, 2.7),
                        Transform::from_rotation(
                            Quat::from_euler(EulerRot::XYZ, 90.0, 90.0, 0.0),
                        ),
                    )
                ]
                .into(),
            },
            //Tetraedron
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 1.65, 0),
                    rotation: q.clone(),
                    scale: 3.5 * v.clone(),
                }),
                shape: mesh_assets.pyramida.as_ref().unwrap().clone(),
                colliders: [DisplacedCollider::bundle(
                    Collider::cone(0.83, 1.0),
                    Transform::default()
                )]
                .into(),
            },
            //Character: Capsule + Sphere
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 1.5, 0),
                    rotation: q.clone(),
                    scale: 1.5 * v.clone(),
                }),
                shape: meshes.add(Capsule3d::default()),
                colliders: [DisplacedCollider::bundle(
                    capsule_y(1.0, 0.5),
                    ..default(),
                )]
                .into(),
            },
            Figure {
                transform: TransformBundle::from_transform(Transform {
                    translation: Vec3::new(0, 3.7, 0),
                    rotation: q.clone(),
                    scale: v.clone(),
                }),
                shape: meshes.add(Sphere::default().mesh().ico(5).unwrap()),
                colliders: [DisplacedCollider::bundle(
                    Collider::ball(0.5),
                    ..default(),
                )]
                .into(),
            },
        ])));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnInitSet;

#[derive(Component, Clone, Debug)]
pub struct Shape{
    pub transform: TransformBundle, 
    pub shape: Handle< Mesh >,
    pub colliders: Vec<DisplacedColliderBundle>
}

#[derive(Clone, Debug, Default, Resource)]
pub struct Shapes(pub Vec<Shape>);

fn map_shapes_transform(
    mut shapes: Res<Shapes>
){
    let shapes_in_cirlce_count = shapes.len() - 2;
    let degrees_step = 360.0 / shapes_in_cirlce_count as f32;
    for (i, shape) in shapes.iter_mut().enumerate(){
        if i < shapes_in_cirlce_count{
            let Vec2 { x, y: z } = polar_to_decart(Vec2::new(7.0, degrees_step * i as f32));
            let translation_y = shape.transform.translation.y;
            shape.transform.translation = Vec3::new(x, translation_y, z);
        }
    }
}

fn spawn_shapes(
    mut commands: Commands,
    mut meshes : ResMut< Assets< Mesh > >,
    mesh_assets: Res< MeshAssets >,
    outline_material : Query< &Handle< ExtendedMaterial< StandardMaterial, outline::Outline< RimEffect > > >, With< RimEffect > >,
    shapes: Res<Shapes>
){
    for shape in shapes.iter( )
    {
        let Shape{ transform, shape, colliders } = shape;
        commands.spawn( 
            MaterialMeshBundle
            {
                mesh : meshes.add(shape),
                transform,
                material : outline_material.single( ).clone( ),
                ..default( )
            }
        ).with_children(|commands| {
            for collider in colliders.iter(){
                commands.spawn(collider);
            }
        });
    }
}

fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_assets: Res<TextureAssets>,
    outline_material: Query<
        &Handle<ExtendedMaterial<StandardMaterial, outline::Outline<RimEffect>>>,
        With<RimEffect>,
    >,
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