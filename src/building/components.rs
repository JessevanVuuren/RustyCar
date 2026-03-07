use bevy::prelude::*;

#[derive(Component)]
pub struct Building {
    pub transform: Transform,
    pub name: String,
    pub path: String,
}
