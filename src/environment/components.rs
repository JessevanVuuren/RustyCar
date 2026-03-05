use bevy::prelude::*;

#[derive(Component)]
pub struct World {
    pub transform: Transform,
    pub model: Model,
}

#[derive(Clone, Component)]
pub struct Model {
    pub nature: Nature,
    pub name: String,
}

#[derive(Clone)]
pub enum Nature {
    Rock,
    Log,
    Tree,
    Ground,
}
