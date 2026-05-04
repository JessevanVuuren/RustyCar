use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

use crate::physics::components::{Collider, Shape};

pub fn add_collider(commands: &mut Commands, id: Entity, collider: Collider) {
    let mut transform = Transform::IDENTITY;

    transform.translation = collider.position;
    transform.rotation = Quat::from_scaled_axis(collider.rotation);

    commands.entity(id).insert(collider);
}

pub fn build_collider(transform: Transform, shape: &Shape) -> Transform {
    let mut transform = transform.clone();

    match shape {
        Shape::Sphere(radius) => transform.scale = Vec3::splat(*radius),
        Shape::Box(size) => transform.scale = *size,
    }

    transform
}
