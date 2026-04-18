use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
    transform,
};

use crate::{
    collision::components::{Collider, ModelCollider, Shape},
    world::components::StaticWorld,
};

pub fn collider_debug(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Shape, &ChildOf), With<Collider>>,
    collider_query: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for (entity, collider_transform, shape, child_of) in query {
        match shape {
            Shape::Sphere(_) => (),
            Shape::Box(size) => {
                if let Ok(parent_transform) = collider_query.get(child_of.parent()) {
                    let mut transform = parent_transform.mul_transform(*collider_transform);
                    transform.scale = *size;
                    gizmos.cube(transform, RED);
                }
            }
            Shape::None => (),
        }
    }
}
