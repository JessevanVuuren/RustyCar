use bevy::prelude::*;

use crate::world::components::StaticWorld;

pub fn init_static_world(mut commands: Commands, static_world: Res<StaticWorld>) {
    println!("{:#?}", static_world);
}
