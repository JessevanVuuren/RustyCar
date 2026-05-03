use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{car::components::Car, physics::utils::build_collider};

#[derive(Debug, Clone)]
pub struct ModelCollider {
    pub position: Vec3,
    pub rotation: Vec3,
    pub shape: Shape,
    pub effect: Effect,
}

#[derive(Component, Debug, Clone)]
pub enum Effect {
    InverseVelocity,
    Bounce,
    Stop,
    Fixed,
}

#[derive(Component)]
pub struct Gravity;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Collision {
    pub depth: f32,
    pub normal: Vec3,
    pub other: Entity,
    pub effect: Effect,
    pub direction: Vec3,
}

impl Collision {
    pub fn new(normal: Vec3, depth: f32, direction: Vec3, other: Entity, effect: Effect) -> Self {
        Collision {
            depth,
            normal,
            direction,
            other,
            effect,
        }
    }

    pub fn update(&mut self, normal: Vec3, depth: f32, direction: Vec3) {
        self.direction = direction;
        self.normal = normal;
        self.depth = depth;
    }
}

#[derive(Component, Debug, Clone)]
pub enum Shape {
    Sphere(f32),
    Box(Vec3),
}
