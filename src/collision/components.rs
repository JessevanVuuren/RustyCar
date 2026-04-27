use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{car::components::Car, collision::utils::build_collider};

#[derive(Debug, Default, Clone)]
pub struct ModelCollider {
    pub position: Vec3,
    pub rotation: Vec3,
    pub shape: Shape,
    pub effect: Effect,
}

#[derive(Component, Default, Debug, Clone)]
pub enum Effect {
    InverseVelocity,
    Bounce,
    Stop,
    #[default]
    Fixed,
}

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

#[derive(Component, Debug, Default, Clone)]
pub enum Shape {
    Sphere(f32),
    Box(Vec3),
    #[default]
    None,
}
