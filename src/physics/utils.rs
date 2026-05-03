use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

use crate::physics::components::{Collider, Effect, ModelCollider, Shape};

pub fn add_collider(commands: &mut Commands, id: Entity, collider: ModelCollider) {
    let mut transform = Transform::IDENTITY;

    transform.translation = collider.position;
    transform.rotation = Quat::from_scaled_axis(collider.rotation);

    commands.entity(id).with_children(|parent| {
        parent.spawn((transform, Collider, collider.shape, collider.effect));
    });
}

pub fn build_collider(a: &Transform, b: &Transform, shape: &Shape) -> Transform {
    let mut transform = a.mul_transform(*b);

    match shape {
        Shape::Sphere(radius) => transform.scale = Vec3::splat(*radius),
        Shape::Box(size) => transform.scale = *size,
    }

    transform
}

pub fn build_colliders<T: Component>(
    collider_query: Query<(Entity, &Transform)>,
    query: Query<(&Transform, &Shape, &ChildOf, &Effect), With<(T)>>,
) -> Vec<(Entity, Transform, Effect, Shape)> {
    let mut colliders = Vec::new();

    for (child, shape, child_of, effect) in query {
        if let Ok((entity, parent)) = collider_query.get(child_of.parent()) {
            let collider = build_collider(parent, child, shape);
            colliders.push((entity, collider, effect.clone(), shape.clone()));
        }
    }

    colliders
}
