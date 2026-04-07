pub mod components;
pub mod spawn;
pub mod systems;

use bevy::prelude::*;

use crate::animal::spawn::spawn_butterfly;

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_butterfly);
    }
}
