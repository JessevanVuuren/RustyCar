use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Wheel {
    pub position: WheelPosition,
    pub offset: Transform,
    pub current: f32,
}

#[derive(Component, Default)]
pub enum WheelPosition {
    #[default]
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

#[derive(Component, Default)]
pub struct Car {
    pub velocity: Vec3,
    pub target: f32,
    pub actual: f32,
}
