use std::collections::HashSet;

use bevy::{
    color::palettes::css::{BLUE, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};

use crate::{
    extra::utils::comma_print,
    physics::{
        components::{Collider, Collision, Effect, Gravity, Shape, Velocity},
        theorem::{collision_box_sphere, separating_axis_theorem, sphere_on_sphere},
        utils::build_collider,
    },
    world::components::StaticWorld,
};

const GRAVITY: f32 = 9.81;

pub fn collider_debug(mut gizmos: Gizmos, query: Query<(&Transform, &Collider)>) {
    for (transform, collider) in query {
        let ct = build_collider(*transform, &collider.shape);

        match collider.shape {
            Shape::Sphere(_) => {
                gizmos.sphere(ct.to_isometry(), ct.scale.x * 0.5, RED);
            }
            Shape::Box(_) => {
                gizmos.cube(ct, RED);
            }
        }
    }
}

pub fn collider_collision_enter(
    mut gizmos: Gizmos,
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Collider)>,
) {
    let colliders: Vec<_> = query.iter().collect();

    for index_a in 0..colliders.len() {
        for index_b in (index_a + 1)..colliders.len() {
            let (entity_a, transform_a, collider_a) = colliders[index_a];
            let (entity_b, transform_b, collider_b) = colliders[index_b];

            let collider_a = collider_a.clone();
            let collider_b = collider_b.clone();

            if matches!(collider_a.effect, Effect::Fixed)
                && matches!(collider_b.effect, Effect::Fixed)
            {
                continue;
            }

            let box_a = &build_collider(*transform_a, &collider_a.shape);
            let box_b = &build_collider(*transform_b, &collider_b.shape);

            let collision = match (collider_a.shape, collider_b.shape) {
                (Shape::Sphere(_), Shape::Sphere(_)) => sphere_on_sphere(box_a, box_b),
                (Shape::Sphere(_), Shape::Box(_)) => collision_box_sphere(box_b, box_a),
                (Shape::Box(_), Shape::Sphere(_)) => collision_box_sphere(box_a, box_b),
                (Shape::Box(_), Shape::Box(_)) => separating_axis_theorem(box_a, box_b),
            };

            if let Some((normal, depth)) = collision {
                let direction = (box_a.translation - box_b.translation).normalize();

                let collision_a =
                    Collision::new(normal, depth, direction, entity_b, collider_a.effect);
                let collision_b =
                    Collision::new(normal, depth, direction, entity_a, collider_b.effect);

                commands.entity(entity_a).insert(collision_a);
                commands.entity(entity_b).insert(collision_b);
            }
        }
    }
}

pub fn current_collisions_depth(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Collider, &mut Collision)>,
) {
    let mut check = HashSet::new();
    let mut pairs = Vec::new();

    for (entity, _, _, collision) in query.iter() {
        let other = collision.other;

        if entity != other && !check.contains(&other) {
            check.insert(entity);
            pairs.push((entity, other));
        }
    }

    for (a, b) in pairs {
        if let Ok([mut a_data, mut b_data]) = query.get_many_mut([a, b]) {
            let (entity_a, transform_a, collider_a, _) = a_data;
            let (entity_b, transform_b, collider_b, _) = b_data;

            let collider_a = collider_a.clone();
            let collider_b = collider_b.clone();

            let box_a = &build_collider(*transform_a, &collider_a.shape);
            let box_b = &build_collider(*transform_b, &collider_b.shape);

            let collision = match (collider_a.shape, collider_b.shape) {
                (Shape::Sphere(_), Shape::Sphere(_)) => sphere_on_sphere(box_a, box_b),
                (Shape::Sphere(_), Shape::Box(_)) => collision_box_sphere(box_b, box_a),
                (Shape::Box(_), Shape::Sphere(_)) => collision_box_sphere(box_a, box_b),
                (Shape::Box(_), Shape::Box(_)) => separating_axis_theorem(box_a, box_b),
            };

            if let Some((normal, depth)) = collision {
                let direction = (box_a.translation - box_b.translation).normalize();

                a_data.3.update(normal, depth, direction);
                b_data.3.update(normal, depth, direction);
            } else {
                commands.entity(a_data.0).remove::<Collision>();
                commands.entity(b_data.0).remove::<Collision>();
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
