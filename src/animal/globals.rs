use std::ops::Range;

use bevy::prelude::*;

// ups = units per second

// natural fly path
pub const FLY_PATH_MAX_UPS: Range<f32> = 2.2..5.4;
pub const FLY_PATH_MAX_REST: Range<f32> = 1.0..10.0;
pub const FLY_PATH_MAX_HEIGHT: Range<f32> = 0.0..1.0;
pub const FLY_PATH_MAX_INTENSITY: Range<f32> = 0.0..0.1;
pub const FLY_PATH_MAX_AMPLITUDE: Range<Vec3> = Vec3::ZERO..Vec3::new(0.5, 1.0, 0.5);
pub const FLY_PATH_MAX_FREQUENCY: Range<Vec3> = Vec3::ZERO..Vec3::new(50.0, 50.0, 50.0);

// free fly
pub const FREE_FLY_HEIGHT: f32 = 3.0;

pub const SWIRLING_MAX_UPS: Range<f32> = 0.5..1.0;
pub const SWIRLING_MAX_HEIGHT: Range<f32> = 1.5..3.0;

pub const SWIRLING_MAX_RADIUS: Range<f32> = 2.0..3.0;

pub const SWIRLING_MAX_RADIUS_AMPLITUDE: Range<f32> = 0.1..0.5;
pub const SWIRLING_MAX_RADIUS_FREQUENCY: Range<f32> = 3.5..10.0;

pub const SWIRLING_MAX_AMPLITUDE: Range<Vec3> = Vec3::new(0.5, 0.3, 0.5)..Vec3::new(0.7, 0.5, 0.7);
pub const SWIRLING_MAX_FREQUENCY: Range<Vec3> = Vec3::new(2.0, 2.0, 2.0)..Vec3::new(2.5, 2.5, 2.5);