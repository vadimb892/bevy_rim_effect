use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{collider::Displaced, Cursor};

/// The entity that the cursor is currently hovering over, if any.
#[derive(Resource, Default)]
pub struct Hovering {
    /// The entity.
    pub entity: Option<Entity>,
}

/// Sets [`Hovering`] entity on colided entity with [`Ray`] of 
/// current [`Cursor`] with help of crate 
/// [`bevy_rapier3d`].
pub(crate) fn hovering_raycast(
    ctx: Res<RapierContext>,
    cursor: Res<Cursor>,

    mut hovering: ResMut<Hovering>,

    cameras: Query<(&Camera, &GlobalTransform)>,

    displaced_colliders: Query<&Parent, With<Displaced>>,
) {
    if let Ok((camera, gtf)) = cameras.get_single() {
        let Some(ray) = camera.viewport_to_world(gtf, cursor.position()) else {
            debug!("Skipping raycast, viewport_to_world failed");
            return;
        };

        if let Some((entity, _)) = ctx.cast_ray(
            ray.origin,
            ray.direction,
            64.0,
            true,
            QueryFilter::new().predicate(&ignore_filter),
        ) {
            let new_entity = if let Ok(parent) = displaced_colliders.get(entity) {
                parent.get()
            } else {
                entity
            };

            if let Some(current_entity) = hovering.entity {
                if current_entity != new_entity {
                    hovering.entity = Some(new_entity);
                }
            } else {
                hovering.entity = Some(new_entity);
            }
        } else if hovering.entity.is_some() {
            hovering.entity = None;
        }
    }
}