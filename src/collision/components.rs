use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct ModelCollider {
    pub position: Vec3,
    pub rotation: Vec3,
    pub shape: Shape,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component, Debug, Default, Clone)]
pub enum Shape {
    Sphere(f32),
    Box(Vec3),
    #[default]
    None,
}
