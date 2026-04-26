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
    let normals_a = collider_normals(collider_a);
    let normals_b = collider_normals(collider_b);
    let crosses = cross_of_normals(&normals_a, &normals_b);

    let mut axes = Vec::with_capacity(15);
    axes.extend(normals_a);
    axes.extend(normals_b);
    axes.extend(crosses);

    let points_a = transform_shape(collider_a);
    let points_b = transform_shape(collider_b);

    for axis in axes {
        let axis = axis.normalize();

        let (min_a, max_a) = project_points(&points_a, axis);
        let (min_b, max_b) = project_points(&points_b, axis);

        if !overlap(min_a, max_a, min_b, max_b) {
            return false;
        }
    }

    true
}

pub fn cross_of_normals(normals_a: &[Vec3], normals_b: &[Vec3]) -> Vec<Vec3> {
    let len_a = normals_a.len();
    let len_b = normals_b.len();

    let mut crosses = Vec::with_capacity(len_a * len_b);

    for i in 0..normals_a.len() {
        for j in 0..normals_b.len() {
            let axis = normals_a[i].cross(normals_b[j]);
            if axis.length_squared() > 1e-6 {
                crosses.push(axis.normalize());
            }
        }
    }

    crosses
}

pub fn project_points(points: &[Vec3], axis: Vec3) -> (f32, f32) {
    let mut min = points[0].dot(axis);
    let mut max = min;

    for p in points.iter().skip(1) {
        let d = p.dot(axis);
        min = min.min(d);
        max = max.max(d);
    }

    (min, max)
}

pub fn overlap(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> bool {
    min_a <= max_b && min_b <= max_a
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

pub fn collider_normals(transform: &Transform) -> Vec<Vec3> {
    vec![
        transform.right().as_vec3(),
        transform.up().as_vec3(),
        transform.forward().as_vec3(),
    ]
}

pub fn collider(a: &Transform, b: &Transform, size: Vec3) -> Transform {
    let mut transform = a.mul_transform(*b);
    transform.scale = size;
    transform
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

    let normals = collider_normals(transform);

    let right_end = normals[0] * (half.x + 1.0);
    let upward_end = normals[1] * (half.y + 1.0);
    let forward_end = normals[2] * (half.z + 1.0);

    gizmos.line(normals[0] * half.x + center, right_end + center, RED);
    gizmos.line(normals[1] * half.y + center, upward_end + center, GREEN);
    gizmos.line(normals[2] * half.z + center, forward_end + center, BLUE);
}
