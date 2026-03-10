pub mod components;
pub mod spawn;
pub mod utils;

use bevy::prelude::*;

use crate::world::{components::TileWorld, spawn::{generate_fencing, generate_flowers, generate_ground}};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileWorld>();
        app.add_systems(
            Startup,
            ((generate_ground, generate_fencing), generate_flowers).chain(),
        );
    }
}
