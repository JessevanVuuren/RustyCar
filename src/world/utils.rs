use bevy::{math::Quat, transform::components::Transform};
use rand::{RngExt, rngs::SmallRng};

use std::f32::consts::FRAC_PI_2;

pub fn transform_random_direction(rng: &mut SmallRng, transform: &mut Transform) {
    let angle = rng.random_range(0..4);
    transform.rotation = Quat::from_rotation_y(FRAC_PI_2 * angle as f32);
}

pub const DOWN: f32 = 0.0;
pub const UP: f32 = FRAC_PI_2;
pub const LEFT: f32 = FRAC_PI_2 * 3.0;
pub const RIGHT: f32 = FRAC_PI_2 * 2.0;

pub fn transform_with_direction(direction: f32, transform: &mut Transform) {
    transform.rotation = Quat::from_rotation_y(direction);
}
