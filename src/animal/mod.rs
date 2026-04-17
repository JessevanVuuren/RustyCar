pub mod butterfly;
pub mod components;
pub mod globals;
pub mod spawn;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::{
    animal::{
        butterfly::{
            butterfly::{
                // butterfly_animate_flowerbed, butterfly_assign_flowerbed_flower, debug_butterfly_path, update_rest_timer,
                butterfly_animate_freefly, butterfly_assign_freefly_target, debug_butterfly_path, update_rest_timer
            },
            spawn::spawn_butterfly,
        },
        components::AnimalLibrary,
        spawn::spawn_animations,
        systems::{link_animal_animations, update_animal_animations},
    },
    world::components::StaticWorld,
};

pub struct AnimalPlugin {
    pub static_world: StaticWorld,
}

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimalLibrary>();
        app.add_systems(Startup, (spawn_animations, (spawn_butterfly)).chain());
        app.add_systems(Update, (link_animal_animations, update_animal_animations));
        app.add_systems(Update, (update_rest_timer));
        // app.add_systems(Update, (butterfly_assign_flowerbed_flower, butterfly_animate_flowerbed));
        app.add_systems(
            Update,
            (butterfly_assign_freefly_target, butterfly_animate_freefly),
        );

        // debug helper functions
        // app.add_systems(Update, debug_butterfly_path);
    }
}
