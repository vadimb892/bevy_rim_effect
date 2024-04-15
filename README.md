# Rim effect outline

Capstone project that recreates rim effect outline. This project is experiment with [Bevy](https://bevyengine.org/) rust engine for learning purpose. Can be used as lib for adding rim effect outline material extension.

Run demo example where you can change:
- width(power) - press W + mouse scroll;
- time scale - press T + mouse scroll;
- (const width)/(time dependency width) - press R (don`t hold button, otherwise state will change with high frequency).

![Demo](/images/demo.png)

### How use rim effect outline in own project?

[Demo](/examples/demo) example describe how can be integrated rim effect outline in own project.

Base tips to add rim effect outline:

1. Add `Outline<RimEffect>` as `MaterialExtension`([link to code](/examples/demo/outlines.rs)):

```rust
    impl Plugin for OutlinesPlugin 
    {
        fn build(&self, app: &mut App)
        {
            add_outline::< RimEffect >( app );
        }
    }
    
    fn add_outline< O : OutlineLabel >( app : &mut App )
    where 
        Outline< O > : MaterialExtension,
        MaterialPlugin::< ExtendedMaterial< StandardMaterial, Outline< O > > >: Plugin
    {
        O::load_shader(app);

        app.add_plugins(
                MaterialPlugin::< ExtendedMaterial< StandardMaterial, Outline< O > > >::default( )
            )
            .register_type::< Outline< O > >();
    }
```

2. Add systems for changing `Outline<RimEffect>` parameters ([link to code](/examples/demo/outlines.rs)):

```rust
    impl Plugin for OutlinesPlugin 
    {
        fn build(&self, app: &mut App)
        {
            add_outline::< RimEffect >( app );
        }
    }
    
    fn add_outline< O : OutlineLabel >( app : &mut App )
    where 
        Outline< O > : MaterialExtension,
        MaterialPlugin::< ExtendedMaterial< StandardMaterial, Outline< O > > >: Plugin
    {
        app.add_systems( Update, ( 
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
```

3. Add `Outline<RimEffect>` extension to `MaterialExtension` when entity spawned ([link to code](/examples/demo/world.rs)):

```rust
    fn spawn_shapes(
        mut commands: Commands,
        mut materials: ResMut<Assets<ExtendedMaterial< StandardMaterial, Outline< RimEffect > >>>,
        shapes: Res<Shapes>
    ){
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
```

### Build demo

```
make demo
```

or

```
cargo run --example demo
```