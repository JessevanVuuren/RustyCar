pub mod components;
pub mod ground;
pub mod spawn;
pub mod utils;

use bevy::prelude::*;

use crate::world::{
    components::{StaticWorld, TileWorld},
    ground::{color_fade::color_fade, ground_fade::ground_fade},
    spawn::init_static_world,
};

pub struct WorldPlugin {
    pub static_world: StaticWorld,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileWorld>();

        app.insert_resource(self.static_world.clone()).add_systems(
            Startup,
            (init_static_world, (ground_fade, color_fade)).chain(),
        );
    }
}
