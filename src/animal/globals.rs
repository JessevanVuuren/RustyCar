use std::ops::Range;

use bevy::prelude::*;

// ups = units per second

pub const MIN_DIST: f32 = 0.1;

pub const FLY_PATH_MAX_HEIGHT: Range<f32> = 0.0..1.0;
pub const FLY_PATH_MAX_AMPLITUDE: Range<Vec3> = Vec3::ZERO..Vec3::new(0.5, 1.0, 0.5);
pub const FLY_PATH_MAX_FREQUENCY: Range<Vec3> = Vec3::ZERO..Vec3::new(10.0, 10.0, 10.0);
pub const FLY_PATH_MAX_INTENSITY: Range<f32> = 0.0..0.1;
pub const FLY_PATH_MAX_UPS: Range<f32> = 2.2..5.4;
pub const FLY_PATH_MAX_REST: Range<f32> = 1.0..10.0;
