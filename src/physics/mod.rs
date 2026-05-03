use crate::{
    physics::{
        effects::collision_effect_response,
        systems::{apply_gravity, collider_collision_enter, collider_debug},
    },
    world::components::StaticWorld,
};
use bevy::prelude::*;

pub mod components;
pub mod effects;
pub mod systems;
pub mod theorem;
pub mod utils;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_gravity,
                collider_collision_enter,
                collision_effect_response,
                collider_debug,
            )
                .chain(),
        );
    }
}
