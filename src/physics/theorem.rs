use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

pub fn sphere_on_sphere(sphere_a: &Transform, sphere_b: &Transform) -> Option<(Vec3, f32)> {
    let radius_a = sphere_a.scale.x * 0.5;
    let radius_b = sphere_b.scale.x * 0.5;

    let delta = sphere_a.translation - sphere_b.translation;
    let dist = delta.length();

    if dist <= radius_a + radius_b {
        let normal = if dist > 0.0001 { delta / dist } else { Vec3::Y };
        let depth = radius_a + radius_b - dist;
        return Some((normal, depth));
    }

    None
}

pub fn collision_box_sphere(box_a: &Transform, sphere_a: &Transform) -> Option<(Vec3, f32)> {
    let sphere_center = sphere_a.translation;
    let box_center = box_a.translation;

    let radius = sphere_a.scale.x * 0.5;

    let axis = collider_normals(box_a);
    let extent = box_a.scale * 0.5;

    let d = sphere_center - box_center;
    let mut closest = box_center;

    for i in 0..3 {
        let dist = d.dot(axis[i]);
        let clamped = dist.clamp(-extent[i], extent[i]);
        closest += axis[i] * clamped;
    }

    let delta = sphere_center - closest;
    let dist2 = delta.length_squared();

    if dist2 <= radius * radius {
        let dist = dist2.sqrt();

        let normal = if dist > 0.0001 {
            delta / dist
        } else {
            best_axis_inside_collider(axis, extent, d)
        };

        let depth = radius - dist;
        return Some((normal, depth));
    }

    None
}

fn best_axis_inside_collider(axis: Vec<Vec3>, extent: Vec3, d: Vec3) -> Vec3 {
    let mut best_axis = axis[0];
    let mut best_dist = f32::MAX;

    for i in 0..3 {
        let dist = d.dot(axis[i]);

        let separation = extent[i] - dist.abs();

        if separation < best_dist {
            best_dist = separation;
            best_axis = if dist > 0.0 { axis[i] } else { -axis[i] };
        }
    }

    best_axis
}

pub fn separating_axis_theorem(
    collider_a: &Transform,
    collider_b: &Transform,
) -> Option<(Vec3, f32)> {
    let normals_a = collider_normals(collider_a);
    let normals_b = collider_normals(collider_b);
    let crosses = cross_of_normals(&normals_a, &normals_b);

    let mut axes = Vec::with_capacity(15);
    axes.extend(normals_a);
    axes.extend(normals_b);
    axes.extend(crosses);

    let points_a = transform_shape(collider_a);
    let points_b = transform_shape(collider_b);

    let mut depth = f32::MAX;
    let mut normal = Vec3::ZERO;

    for axis in axes {
        let axis = axis.normalize();

        let (min_a, max_a) = project_points(&points_a, axis);
        let (min_b, max_b) = project_points(&points_b, axis);

        let overlap = overlap_depth(min_a, max_a, min_b, max_b);

        if overlap <= 0.0 {
            return None;
        }

        if overlap < depth {
            depth = overlap;
            normal = axis;
        }
    }

    Some((normal, depth))
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

pub fn overlap_depth(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> f32 {
    f32::min(max_a, max_b) - f32::max(min_a, min_b)
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
