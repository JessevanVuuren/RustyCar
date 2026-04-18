use std::f32::consts::{FRAC_2_PI, FRAC_PI_2, FRAC_PI_4};

use bevy::prelude::*;

use crate::collision::components::{Collider, ModelCollider};

pub fn add_collider(commands: &mut Commands, id: Entity, collider: ModelCollider) {
    let mut transform = Transform::IDENTITY;

    transform.translation = collider.position;
    transform.rotation = Quat::from_scaled_axis(collider.rotation);

    commands.entity(id).with_children(|parent| {
        parent.spawn((transform, Collider, collider.shape));
    });
}
