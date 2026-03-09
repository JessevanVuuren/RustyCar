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
    Tree,
    Log,
    Ground,
}


#[derive(Component)]
pub struct Infra {
    pub transform: Transform,
    pub name: String,
    pub path: String,
}
