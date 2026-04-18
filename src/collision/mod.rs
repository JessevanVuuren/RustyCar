use crate::{
    collision::{spawn::spawn_colliders, systems::collider_debug},
    world::components::StaticWorld,
};
use bevy::prelude::*;

pub mod components;
pub mod spawn;
pub mod systems;
pub mod utils;

pub struct ColliderPlugin {
    pub static_world: StaticWorld,
}

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_colliders);
        app.add_systems(Update, collider_debug);
    }
}
