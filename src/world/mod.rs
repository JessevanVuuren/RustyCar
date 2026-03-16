pub mod components;
pub mod spawn;
pub mod utils;
pub mod grass;
pub mod patch;


use bevy::prelude::*;

use crate::world::{
    components::{StaticWorld, TileWorld}, patch::patch_ground, spawn::init_static_world
};

pub struct WorldPlugin {
    pub static_world: StaticWorld,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileWorld>();

        app.insert_resource(self.static_world.clone())
            .add_systems(Startup, init_static_world)
            .add_systems(Startup, patch_ground);
    }
}
