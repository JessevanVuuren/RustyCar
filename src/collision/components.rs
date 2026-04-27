use bevy::prelude::*;

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
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub effect: Effect,
}

#[derive(Component, Debug, Default, Clone)]
pub enum Shape {
    Sphere(f32),
    Box(Vec3),
    #[default]
    None,
}
