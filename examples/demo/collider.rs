use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Mark entity as vacant for collider
#[derive(Debug, Default, Component)]
pub struct ColliderPlace;

/// Shortcut for creating colliders that are displaced
#[derive(Debug, Clone, Copy)]
pub struct DisplacedCollider(PhantomData<()>);

impl DisplacedCollider {
    /// Creates a bundle that should be spawned on a child entity
    pub fn bundle(collider: Collider, displacement: Transform) -> DisplacedColliderBundle {
        DisplacedColliderBundle {
            collider,
            displacement: TransformBundle {
                local: displacement,
                ..Default::default()
            },
            marker: Displaced,
        }
    }
}

/// Marks that corresponding component is displaced, so any events should consider a parent entity
#[derive(Debug, Default, Component)]
pub(crate) struct Displaced;

/// Bundle for specifing colliders that are displaced from parent entity origin
#[derive(Debug, Default, Bundle)]
pub struct DisplacedColliderBundle {
    collider: Collider,
    displacement: TransformBundle,
    marker: Displaced,
}