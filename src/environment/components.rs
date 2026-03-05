use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Env {
    pub transform: Transform,
    pub nature: Nature,
    pub name: String,
}

#[derive(Component, Default)]
pub enum Nature {
    #[default]
    Empty,
    Rock,
    Tree,
    Ground,
}
