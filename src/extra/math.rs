use std::ops::Range;

use bevy::prelude::*;
use rand::{RngExt, rngs::SmallRng};

#[inline]
pub fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

#[inline]
pub fn s_curve(x: f32, intensity: f32) -> f32 {
    1.0 / (1.0 + (x / (1.0 - x)).powf(-intensity))
}

#[inline]
pub fn arc(x: f32, intensity: f32) -> f32 {
    (4.0 * x * (1.0 - x)).powf(intensity)
}

#[inline]
pub fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - ((-2.0 * x + 2.0) as f32).powf(3.0) / 2.0
    }
}

#[inline]
pub fn ease_in_out_sine(x: f32) -> f32 {
    -(f32::cos(std::f32::consts::PI * x) - 1.0) / 2.0
}

#[inline]
pub fn ease_in_out_circ(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

#[inline]
pub fn ease_in_quint(x: f32) -> f32 {
    x.powf(4.0)
}

#[inline]
pub fn flat(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.z)
}

#[inline]
pub fn normalized_sin(v: f32) -> f32 {
    (v.sin() + 1.0) * 0.5
}

#[inline]
pub fn random_vec3(rng: &mut SmallRng) -> Vec3 {
    Vec3::new(rng.random(), rng.random(), rng.random())
}

#[inline]
pub fn rand_vec3_range(rng: &mut SmallRng, vec: Range<Vec3>) -> Vec3 {
    Vec3::new(
        rng.random_range(vec.start.x..vec.end.x),
        rng.random_range(vec.start.y..vec.end.y),
        rng.random_range(vec.start.z..vec.end.z),
    )
}
