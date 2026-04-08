pub mod components;
pub mod ground;
pub mod spawn;
pub mod tile_pos;
pub mod utils;

use bevy::prelude::*;

use crate::world::{
    components::StaticWorld,
    ground::{color_fade::color_fade, ground_fade::ground_fade, ground_offset::ground_offset},
    spawn::{spawn_ground::spawn_ground, spawn_models::spawn_models, spawn_patches::spawn_patches},
};

pub struct WorldPlugin {
    pub static_world: StaticWorld,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.static_world.clone()).add_systems(
            Startup,
            (
                (spawn_models, spawn_patches, spawn_ground),
                (ground_fade, color_fade),
                ground_offset,
            )
                .chain(),
        );
    }
}
