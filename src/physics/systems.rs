use std::collections::HashSet;

use bevy::{
    color::palettes::css::{BLUE, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};

use crate::{
    extra::utils::comma_print,
    physics::{
        components::{Collider, Collision, Effect, Gravity, ModelCollider, Shape, Velocity},
        theorem::{collision_box_sphere, separating_axis_theorem},
        utils::{build_collider, build_colliders},
    },
    world::components::StaticWorld,
};

const GRAVITY: f32 = 9.81;

pub fn collider_debug(
    mut gizmos: Gizmos,
    collider_query: Query<(Entity, &Transform)>,
    query: Query<(&Transform, &Shape, &ChildOf, &Effect), With<Collider>>,
) {
    let colliders = build_colliders(collider_query, query);

    for (_, collider, _, shape) in colliders {
        match shape {
            Shape::Sphere(_) => {
                gizmos.sphere(collider.to_isometry(), collider.scale.x * 0.5, RED);
            }
            Shape::Box(_) => {
                gizmos.cube(collider, RED);
            }
        }
    }
}

pub fn collider_collision_enter(
    mut gizmos: Gizmos,
    mut commands: Commands,
    collider_query: Query<(Entity, &Transform)>,
    query: Query<(&Transform, &Shape, &ChildOf, &Effect), With<Collider>>,
) {
    let colliders = build_colliders(collider_query, query);

    let amount = colliders.len();
    for index_a in 0..amount {
        for index_b in (index_a + 1)..amount {
            let (entity_a, collider_a, effect_a, shape_a) = &colliders[index_a];
            let (entity_b, collider_b, effect_b, shape_b) = &colliders[index_b];

            let collision = match (shape_a, shape_b) {
                (Shape::Sphere(_), Shape::Sphere(_)) => todo!("Sphere on sphere collision"),
                (Shape::Sphere(_), Shape::Box(_)) => collision_box_sphere(collider_b, collider_a),
                (Shape::Box(_), Shape::Sphere(_)) => collision_box_sphere(collider_a, collider_b),
                (Shape::Box(_), Shape::Box(_)) => separating_axis_theorem(&collider_a, &collider_b),
            };

            if let Some((normal, depth)) = collision {
                let direction = (collider_a.translation - collider_b.translation).normalize();

                let collision_a =
                    Collision::new(normal, depth, direction, *entity_b, effect_a.clone());
                let collision_b =
                    Collision::new(normal, depth, direction, *entity_a, effect_b.clone());

                commands.entity(*entity_a).insert(collision_a);
                commands.entity(*entity_b).insert(collision_b);
            }
        }
    }
}

pub fn apply_gravity(time: Res<Time>, query: Query<(&Gravity, &mut Transform, &mut Velocity)>) {
    for (_, mut transform, mut velocity) in query {
        velocity.0 += Vec3::NEG_Y * GRAVITY * time.delta_secs();
        transform.translation += velocity.0 * time.delta_secs();
    }
}
