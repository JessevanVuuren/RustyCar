use bevy::prelude::*;

#[derive(Component)]
pub struct Infra {
    pub transform: Transform,
    pub name: String,
    pub path: String,
}
