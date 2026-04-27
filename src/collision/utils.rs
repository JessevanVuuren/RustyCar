use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

use crate::collision::components::{Collider, Effect, ModelCollider, Shape};

pub fn add_collider(commands: &mut Commands, id: Entity, collider: ModelCollider) {
    let mut transform = Transform::IDENTITY;

    transform.translation = collider.position;
    transform.rotation = Quat::from_scaled_axis(collider.rotation);

    commands.entity(id).with_children(|parent| {
        parent.spawn((transform, Collider, collider.shape, collider.effect));
    });
}

pub fn collider(a: &Transform, b: &Transform, size: Vec3) -> Transform {
    let mut transform = a.mul_transform(*b);
    transform.scale = size;
    transform
}

pub fn build_colliders(
    collider_query: Query<(Entity, &Transform)>,
    query: Query<(&Transform, &Shape, &ChildOf, &Effect), With<Collider>>,
) -> Vec<(Entity, Transform, Effect)> {
    let mut colliders = Vec::new();

    for (child, shape, child_of, effect) in query {
        match shape {
            Shape::Sphere(_) => todo!("Implement Sphere collider"),
            Shape::Box(size) => {
                if let Ok((entity, parent)) = collider_query.get(child_of.parent()) {
                    colliders.push((entity, collider(parent, child, *size), effect.clone()));
                }
            }
            Shape::None => todo!("Collider Shape must be implemented"),
        }
    }

    colliders
}
