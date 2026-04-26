use bevy::{
    color::palettes::css::{BLUE, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};

use crate::{
    collision::{
        components::{Collider, ModelCollider, Shape},
        utils::{
            collider, collider_normals, debug_corner, debug_corners, debug_xyz_normals,
            min_max_vectors, separating_axis_theorem,
        },
    },
    world::components::StaticWorld,
};

pub fn collider_debug(
    query: Query<(Entity, &Transform, &Shape, &ChildOf), With<Collider>>,
    collider_query: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for (entity, child, shape, child_of) in query {
        match shape {
            Shape::Sphere(_) => (),
            Shape::Box(size) => {
                if let Ok(parent) = collider_query.get(child_of.parent()) {
                    let collider = collider(parent, child, *size);

                    gizmos.cube(collider, RED);
                }
            }
            Shape::None => (),
        }
    }
}

pub fn collider_collision_check(
    mut gizmos: Gizmos,
    collider_query: Query<&Transform>,
    query: Query<(Entity, &Transform, &Shape, &ChildOf), With<Collider>>,
) {
    let mut colliders = Vec::new();

    for (entity, child, shape, child_of) in query {
        match shape {
            Shape::Sphere(_) => (),
            Shape::Box(size) => {
                if let Ok(parent) = collider_query.get(child_of.parent()) {
                    colliders.push(collider(parent, child, *size));
                }
            }
            Shape::None => (),
        }
    }

    let amount = colliders.len();
    for index_a in 0..amount {
        for index_b in (index_a + 1)..amount {
            let collider_a = colliders[index_a];
            let collider_b = colliders[index_b];

            let result = separating_axis_theorem(&collider_a, &collider_b);

            println!("Collide: {:#?}", result);
        }
    }
}
