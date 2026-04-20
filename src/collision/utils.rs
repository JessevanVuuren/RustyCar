use std::f32::consts::{FRAC_2_PI, FRAC_PI_2, FRAC_PI_4};

use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

use crate::collision::components::{Collider, ModelCollider};

pub fn add_collider(commands: &mut Commands, id: Entity, collider: ModelCollider) {
    let mut transform = Transform::IDENTITY;

    transform.translation = collider.position;
    transform.rotation = Quat::from_scaled_axis(collider.rotation);

    commands.entity(id).with_children(|parent| {
        parent.spawn((transform, Collider, collider.shape));
    });
}

pub fn separating_axis_theorem(collider_a: &Transform, collider_b: &Transform) -> bool {
    let (x_norm_a, y_norm_a, z_norm_a) = collider_normals(collider_a);
    let (x_norm_b, y_norm_b, z_norm_b) = collider_normals(collider_b);

    let points_a = points_along_projection(&collider_a, x_norm_a);
    let points_b = points_along_projection(&collider_b, x_norm_a);

    let (min_a, max_a) = min_max_vectors(&points_a);
    let (min_b, max_b) = min_max_vectors(&points_b);

    lines_overlap(min_a, max_a, min_b, max_b)
}

pub fn lines_overlap(start_a: Vec3, stop_a: Vec3, start_b: Vec3, stop_b: Vec3) -> bool {
    let a_min = start_a.x.min(stop_a.x);
    let a_max = start_a.x.max(stop_a.x);

    let b_min = start_b.x.min(stop_b.x);
    let b_max = start_b.x.max(stop_b.x);

    a_max >= b_min && b_max >= a_min
}

pub fn points_along_projection(transform: &Transform, vector: Vec3) -> Vec<Vec3> {
    let points = transform_shape(transform);
    points
        .iter()
        .map(|point| point.dot(vector) * vector)
        .collect()
}

pub fn min_max_vectors(vectors: &Vec<Vec3>) -> (Vec3, Vec3) {
    let mut low = vectors[0];
    let mut max = vectors[0];

    for vector in vectors {
        let length = vector.length();

        if length < low.length() {
            low = *vector;
        }

        if length > max.length() {
            max = *vector;
        }
    }

    (low, max)
}

pub fn collider_normals(transform: &Transform) -> (Vec3, Vec3, Vec3) {
    (
        transform.right().as_vec3(),
        transform.up().as_vec3(),
        transform.forward().as_vec3(),
    )
}

pub fn transform_shape(transform: &Transform) -> [Vec3; 8] {
    let rotation = transform.rotation;
    let center = transform.translation;
    let half = transform.scale / 2.0;

    [
        rotation.mul_vec3(Vec3::new(-half.x, -half.y, -half.z)) + center,
        rotation.mul_vec3(Vec3::new(half.x, -half.y, -half.z)) + center,
        rotation.mul_vec3(Vec3::new(-half.x, half.y, -half.z)) + center,
        rotation.mul_vec3(Vec3::new(half.x, half.y, -half.z)) + center,
        rotation.mul_vec3(Vec3::new(-half.x, -half.y, half.z)) + center,
        rotation.mul_vec3(Vec3::new(half.x, -half.y, half.z)) + center,
        rotation.mul_vec3(Vec3::new(-half.x, half.y, half.z)) + center,
        rotation.mul_vec3(Vec3::new(half.x, half.y, half.z)) + center,
    ]
}

pub fn debug_corners(gizmos: &mut Gizmos, transform: &Transform, radius: f32, color: Srgba) {
    let corners = transform_shape(transform);

    for c in corners {
        debug_corner(gizmos, c, radius, color);
    }
}

pub fn debug_corner(gizmos: &mut Gizmos, c: Vec3, radius: f32, color: Srgba) {
    let pos = Isometry3d::from_xyz(c.x, c.y, c.z);
    gizmos.sphere(pos, radius, color);
}

pub fn debug_xyz_normals(gizmos: &mut Gizmos, transform: &Transform) {
    let center = transform.translation;
    let half = transform.scale / 2.0;

    let (right, upward, forward) = collider_normals(transform);

    let right_end = right * (half.x + 1.0);
    let upward_end = upward * (half.y + 1.0);
    let forward_end = forward * (half.z + 1.0);

    gizmos.line(right * half.x + center, right_end + center, RED);
    gizmos.line(upward * half.y + center, upward_end + center, GREEN);
    gizmos.line(forward * half.z + center, forward_end + center, BLUE);
}

pub fn collider(a: &Transform, b: &Transform, size: Vec3) -> Transform {
    let mut transform = a.mul_transform(*b);
    transform.scale = size;
    transform
}
