use std::time::Duration;

use crate::{
    animal::globals::{
        FLY_PATH_MAX_AMPLITUDE, FLY_PATH_MAX_FREQUENCY, FLY_PATH_MAX_HEIGHT,
        FLY_PATH_MAX_INTENSITY, FLY_PATH_MAX_REST, FLY_PATH_MAX_UPS,
    },
    extra::math::{arc, normalized_sin, rand_vec3_range, random_vec3},
};
use bevy::prelude::*;
use rand::{RngExt, rngs::SmallRng};

#[derive(Component, Default)]
pub struct NaturalFlyPath {
    front: Vec3,
    right: Vec3,

    start: Vec3,
    stop: Vec3,

    frequency: Vec3,
    amplitude: Vec3,

    intensity: f32,
    distance: f32,
    height: f32,
    speed: f32,
    rest: f32,
    step: f32,
}

impl NaturalFlyPath {
    pub fn max_values(rng: &mut SmallRng, start: Vec3, stop: Vec3) -> Self {
        Self::pre_compute(
            FLY_PATH_MAX_FREQUENCY.end,
            FLY_PATH_MAX_AMPLITUDE.end,
            FLY_PATH_MAX_INTENSITY.end,
            FLY_PATH_MAX_HEIGHT.end,
            FLY_PATH_MAX_UPS.end,
            FLY_PATH_MAX_REST.end,
            start,
            stop,
        )
    }

    pub fn min_values(rng: &mut SmallRng, start: Vec3, stop: Vec3) -> Self {
        Self::pre_compute(
            FLY_PATH_MAX_FREQUENCY.start,
            FLY_PATH_MAX_AMPLITUDE.start,
            FLY_PATH_MAX_INTENSITY.start,
            FLY_PATH_MAX_HEIGHT.start,
            FLY_PATH_MAX_UPS.start,
            FLY_PATH_MAX_REST.start,
            start,
            stop,
        )
    }

    pub fn random(rng: &mut SmallRng, start: Vec3, stop: Vec3) -> Self {
        Self::pre_compute(
            rand_vec3_range(rng, FLY_PATH_MAX_FREQUENCY),
            rand_vec3_range(rng, FLY_PATH_MAX_AMPLITUDE),
            rng.random_range(FLY_PATH_MAX_INTENSITY),
            rng.random_range(FLY_PATH_MAX_HEIGHT),
            rng.random_range(FLY_PATH_MAX_UPS),
            rng.random_range(FLY_PATH_MAX_REST),
            start,
            stop,
        )
    }

    fn pre_compute(
        frequency: Vec3,
        amplitude: Vec3,
        intensity: f32,
        height: f32,
        speed: f32,
        rest: f32,
        start: Vec3,
        stop: Vec3,
    ) -> Self {
        let distance = start.distance(stop);
        let speed = 1.0 / distance * speed;
        let intensity = distance * intensity;

        let direction = (start - stop).normalize();
        let right = direction.cross(Vec3::Y).normalize();

        NaturalFlyPath {
            front: direction,
            frequency,
            amplitude,
            intensity,
            distance,
            height,
            step: 0.0,
            right,
            speed,
            start,
            stop,
            rest,
        }
    }

    pub fn step(&mut self, delta_time: f32) {
        self.step += self.speed * delta_time;
    }

    pub fn is_finished(&self) -> bool {
        self.step >= 1.0
    }

    pub fn rest_timer(&self) -> Timer {
        Timer::from_seconds(self.rest, TimerMode::Once)
    }

    pub fn units_per_sec(&self, time: Duration) -> f32 {
        self.distance / time.as_secs_f32()
    }

    pub fn position(&self) -> Vec3 {
        self.sample_at_point(self.step)
    }

    pub fn sample(&self, step: f32) -> Vec3 {
        self.sample_at_point(step)
    }

    fn sample_at_point(&self, step: f32) -> Vec3 {
        let mut pos = self.start.lerp(self.stop, step);
        let arc = arc(step, 1.0) * self.intensity;

        let freq = self.frequency;
        let ampl = self.amplitude;

        let u2d = normalized_sin(step * freq.y) * ampl.y * arc;
        let s2s = (step * freq.x).sin() * ampl.x * arc;
        let f2b = (step * freq.z).cos() * ampl.z * arc;

        pos.y += self.height * arc;
        pos.y += u2d;

        pos += s2s * self.right;
        pos += f2b * self.front;

        pos
    }

    pub fn look_at(&self, pos: Vec3) -> Vec3 {
        let mut target = self.stop;
        target.y = pos.y;
        target
    }
}
