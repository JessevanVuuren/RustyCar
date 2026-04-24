use bevy::{
    color::palettes::css::{BLUE, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};

use crate::{
    collision::{
        components::{Collider, ModelCollider, Shape},
        utils::{
            collider, collider_normals, debug_corner, debug_corners, debug_xyz_normals,
            lines_overlap, min_max_vectors, points_along_projection, separating_axis_theorem,
        },
    },
    world::components::StaticWorld,
};

pub fn collider_debug(
    query: Query<(Entity, &Transform, &Shape, &ChildOf), With<Collider>>,
    collider_query: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    // for (entity, child, shape, child_of) in query {
    //     match shape {
    //         Shape::Sphere(_) => (),
    //         Shape::Box(size) => {
    //             if let Ok(parent) = collider_query.get(child_of.parent()) {
    //                 let collider = collider(parent, child, *size);

    //                 // gizmos.cube(collider, RED);
    //                 // debug_corners(&mut gizmos, &collider, 0.1, YELLOW);

    //                 // let unit = Vec3::new(1.0, 0.0, 0.0);
    //                 // let points = points_along_projection(&collider, unit);
    //                 // let (min, max) = min_max_vectors(&points);

    //                 // debug_corner(&mut gizmos, min, 0.1, YELLOW);
    //                 // debug_corner(&mut gizmos, max, 0.1, YELLOW);

    //                 // debug_xyz_normals(&mut gizmos, &collider);
    //             }
    //         }
    //         Shape::None => (),
    //     }
    // }
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

            // let result = separating_axis_theorem(&collider_a, &collider_b);

            gizmos.cube(collider_b, RED);
            debug_xyz_normals(&mut gizmos, &collider_b);

            let (x_norm_a, y_norm_a, z_norm_a) = collider_normals(&collider_a);
            let (x_norm_b, y_norm_b, z_norm_b) = collider_normals(&collider_b);

            let points_a = points_along_projection(&collider_a, z_norm_a);
            let points_b = points_along_projection(&collider_b, z_norm_a);

            let (min_a, max_b) = min_max_vectors(&points_a);
            let (min_c, max_d) = min_max_vectors(&points_b);

            debug_corner(&mut gizmos, min_a, 0.1, YELLOW);
            debug_corner(&mut gizmos, max_b, 0.1, YELLOW);

            debug_corner(&mut gizmos, min_c, 0.1, PURPLE);
            debug_corner(&mut gizmos, max_d, 0.1, PURPLE);

            if lines_overlap(min_a, max_b, min_c, max_d, z_norm_a) {
                println!("Overlap")
            } else {
                println!("None")
            }

            // println!("collide: {}", result);
        }
    }
}
