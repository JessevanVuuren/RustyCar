pub mod behaviors;
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
                butterfly_assign_flowerbed_flower, butterfly_assign_freefly_target,
                butterfly_finish_flowerbed, butterfly_finish_freefly, butterfly_swirl,
            },
            spawn::spawn_butterfly,
        },
        components::AnimalLibrary,
        spawn::spawn_animations,
        systems::{
            animal_animate_natural_fly_path, link_animal_animations, update_animal_animations,
            update_rest_timer,
        },
    },
    world::components::StaticWorld,
};

pub struct AnimalPlugin {
    pub static_world: StaticWorld,
}

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimalLibrary>();
        app.add_systems(Startup, (spawn_animations, spawn_butterfly).chain());
        app.add_systems(Update, (link_animal_animations, update_animal_animations));
        app.add_systems(Update, (update_rest_timer, animal_animate_natural_fly_path));
        app.add_systems(
            Update,
            (
                butterfly_assign_freefly_target,
                butterfly_assign_flowerbed_flower,
                butterfly_finish_freefly,
                butterfly_finish_flowerbed,
                butterfly_swirl,
            ),
        );
    }
}
