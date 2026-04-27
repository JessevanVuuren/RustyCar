use bevy::{
    color::palettes::css::{BLUE, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};

use crate::{
    collision::{
        components::{Collider, Collision, Effect, ModelCollider, Shape},
        theorem::separating_axis_theorem,
        utils::build_colliders,
    },
    world::components::StaticWorld,
};

pub fn collider_debug(
    mut gizmos: Gizmos,
    collider_query: Query<(Entity, &Transform)>,
    query: Query<(&Transform, &Shape, &ChildOf, &Effect), With<Collider>>,
) {
    let colliders = build_colliders(collider_query, query);

    for (_, collider, _) in colliders {
        gizmos.cube(collider, RED);
    }
}

pub fn collider_collision(
    mut gizmos: Gizmos,
    mut commands: Commands,
    collider_query: Query<(Entity, &Transform)>,
    query: Query<(&Transform, &Shape, &ChildOf, &Effect), With<Collider>>,
) {
    let colliders = build_colliders(collider_query, query);

    let amount = colliders.len();
    for index_a in 0..amount {
        for index_b in (index_a + 1)..amount {
            let (entity_a, collider_a, effect_a) = &colliders[index_a];
            let (entity_b, collider_b, effect_b) = &colliders[index_b];

            if let Some((normal, depth)) = separating_axis_theorem(&collider_a, &collider_b) {
                let direction = (collider_a.translation - collider_b.translation).normalize();
                let collision_a = Collision {
                    depth,
                    normal,
                    direction,
                    other: *entity_b,
                    effect: effect_a.clone(),
                };

                let collision_b = Collision {
                    depth,
                    normal,
                    direction,
                    other: *entity_a,
                    effect: effect_b.clone(),
                };

                commands.entity(*entity_a).insert(collision_a);
                commands.entity(*entity_b).insert(collision_b);
            }
        }
    }
}
