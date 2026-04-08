pub mod animals;
pub mod components;
pub mod globals;
pub mod spawn;
pub mod systems;

use bevy::prelude::*;

use crate::{
    animal::{
        animals::butterfly::{animate_butterfly, butterfly_assign_flower, update_rest_timer},
        components::AnimalLibrary,
        spawn::{spawn_animals, spawn_animations},
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
        app.add_systems(Startup, (spawn_animations, spawn_animals).chain());
        app.add_systems(Update, (link_animal_animations, update_animal_animations));
        app.add_systems(Update, (update_rest_timer));
        app.add_systems(Update, (butterfly_assign_flower, animate_butterfly));
    }
}
